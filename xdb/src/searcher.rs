use std::{ error::Error, fmt::Display, fs::File, io::Read, path::Path };
use once_cell::sync::OnceCell;
use tracing::error;
use crate::ToUIntIP;
const HEADER_INFO_LENGTH: usize = 256;
const VECTOR_INDEX_COLS: usize = 256;
const VECTOR_INDEX_SIZE: usize = 8;
const SEGMENT_INDEX_SIZE: usize = 14;
const VECTOR_INDEX_LENGTH: usize = 512 * 1024;
const XDB_FILEPATH_ENV: &str = "XDB_FILEPATH";
// 只保留一个缓存，用于存储完整数据
static CACHE: OnceCell<Vec<u8>> = OnceCell::new();
// 添加专门的向量缓存
static VECTOR_CACHE: OnceCell<&'static [u8]> = OnceCell::new();
// 默认xdb位置
fn default_detect_xdb_file() -> Result<String, Box<dyn Error>> {
    let prefix = "../".to_owned();
    for recurse in 1..4 {
        let filepath = prefix.repeat(recurse) + "server/static/ip2region.xdb";
        if Path::new(&filepath).exists() {
            return Ok(filepath);
        }
    }
    Err("default filepath not find the xdb file".into())
}

#[inline(always)]
pub fn get_block_by_size(bytes: &[u8], offset: usize, length: usize) -> usize {
    unsafe {
        let ptr = bytes.as_ptr().add(offset);
        match length {
            4 => {
                let mut buf = [0u8; 4];
                std::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), 4);
                u32::from_ne_bytes(buf) as usize
            }
            2 => {
                let mut buf = [0u8; 2];
                std::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), 2);
                u16::from_ne_bytes(buf) as usize
            }
            _ => {
                let mut result = 0usize;
                std::ptr::copy_nonoverlapping(ptr, &mut result as *mut usize as *mut u8, length);
                result
            }
        }
    }
}

#[inline(always)]
pub fn search_by_ip<T>(ip: T) -> Result<String, Box<dyn Error>> where T: ToUIntIP + Display {
    let ip = ip.to_u32_ip()?;

    unsafe {
        let vector_cache = get_vector_index_cache();
        let offset =
            VECTOR_INDEX_SIZE *
            ((((ip >> 24) & 0xff) as usize) * VECTOR_INDEX_COLS + (((ip >> 16) & 0xff) as usize));

        // 使用 get_block_by_size 来安全地读取数据
        let start_ptr = get_block_by_size(vector_cache, offset, 4);
        let end_ptr = get_block_by_size(vector_cache, offset + 4, 4);

        let full_cache = get_full_cache();
        let cache_ptr = full_cache.as_ptr().add(start_ptr);
        let mut left = 0;
        let mut right = (end_ptr - start_ptr) / SEGMENT_INDEX_SIZE;

        while left < right {
            let mid = (left + right) >> 1;
            let segment_offset = mid * SEGMENT_INDEX_SIZE;
            let _segment_ptr = cache_ptr.add(segment_offset);

            // 使用 get_block_by_size 读取所有数据
            let start_ip = get_block_by_size(full_cache, start_ptr + segment_offset, 4) as u32;
            if ip < start_ip {
                right = mid;
                continue;
            }

            let end_ip = get_block_by_size(full_cache, start_ptr + segment_offset + 4, 4) as u32;
            if ip > end_ip {
                left = mid + 1;
                continue;
            }

            let data_len = get_block_by_size(full_cache, start_ptr + segment_offset + 8, 2);
            let data_offset = get_block_by_size(full_cache, start_ptr + segment_offset + 10, 4);

            let data = std::slice::from_raw_parts(full_cache.as_ptr().add(data_offset), data_len);

            return Ok(String::from_utf8_unchecked(data.to_vec()));
        }

        Err("not matched".into())
    }
}

// 优化向量索引缓存访问
#[inline(always)]
pub fn get_vector_index_cache() -> &'static [u8] {
    // 使用静态缓存，只计算一次
    VECTOR_CACHE.get_or_init(|| unsafe {
        let full_cache = get_full_cache();
        let ptr = full_cache.as_ptr().add(HEADER_INFO_LENGTH);
        std::slice::from_raw_parts(ptr, VECTOR_INDEX_LENGTH)
    })
}
//
#[inline(always)]
pub fn get_full_cache() -> &'static Vec<u8> {
    CACHE.get_or_init(|| {
        let xdb_filepath = std::env
            ::var(XDB_FILEPATH_ENV)
            .unwrap_or_else(|_| default_detect_xdb_file().unwrap());

        let size = std::fs
            ::metadata(&xdb_filepath)
            .map(|m| m.len() as usize)
            .unwrap_or(1024 * 1024);

        let mut buffer = Vec::with_capacity(size);
        File::open(&xdb_filepath)
            .expect("file open error")
            .read_to_end(&mut buffer)
            .expect("load file error");

        buffer
    })
}

pub fn searcher_init(xdb_filepath: Option<String>) {
    let xdb_filepath = xdb_filepath.unwrap_or_else(||
        default_detect_xdb_file().expect("打开xdb_file出错")
    );

    unsafe {
        std::env::set_var(XDB_FILEPATH_ENV, xdb_filepath);
    }

    // 初始化并预热两个缓存
    let _ = get_full_cache();
    let _ = get_vector_index_cache();
}

#[cfg(test)]
mod tests {
    use std::{ fs::File, io::Read, net::Ipv4Addr, str::FromStr, thread };

    use super::*;

    /// test all types find correct
    #[test]
    fn test_multi_type_ip() {
        searcher_init(None);

        search_by_ip("2.0.0.0").unwrap();
        search_by_ip("32").unwrap();
        search_by_ip(4294408949).unwrap();
        search_by_ip(Ipv4Addr::from_str("1.1.1.1").unwrap()).unwrap();
    }

    #[test]
    fn test_match_all_ip_correct() {
        searcher_init(None);
        let mut file = File::open("../server/static/ip.test.txt").expect(
            "无法打开测试数据文件，请确保static/ip.test.txt存在"
        );
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for line in contents.split("\n") {
            if !line.contains("|") {
                continue;
            }
            let ip_test_line = line.splitn(3, "|").collect::<Vec<&str>>();
            let start_ip = Ipv4Addr::from_str(ip_test_line[0]).expect("获取start_ip出错");
            let end_ip = Ipv4Addr::from_str(ip_test_line[1]).expect("获取end_ip出错");
            for value in u32::from(start_ip)..u32::from(end_ip) + 1 {
                let result = search_by_ip(value).unwrap();
                assert_eq!(result.as_str(), ip_test_line[2]);
            }
        }
    }

    #[test]
    fn test_multi_thread_only_load_xdb_once() {
        searcher_init(None);
        let handle = thread::spawn(|| {
            let result = search_by_ip("2.2.2.2").unwrap();
            println!("ip search in spawn: {result}");
        });
        let r = search_by_ip("1.1.1.1").unwrap();
        println!("ip search in main thread: {r}");
        handle.join().unwrap();
    }

    #[test]
    fn test_multi_searcher_init() {
        for _ in 0..5 {
            thread::spawn(|| {
                searcher_init(None);
            });
        }
        searcher_init(None);
        searcher_init(Some(String::from("test")));
        search_by_ip(123).unwrap();
    }
}
