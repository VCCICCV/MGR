// use std::{collections::HashSet, error::Error};

// use bb8::Pool;
// use bb8_redis::RedisConnectionManager;
// use redis::AsyncCommands;

// use crate::database::redis_connection::establish_redis_pool_connection;
// // 定义用于表示数据类型的枚举
// #[derive(Debug)]
// pub enum DataType {
//     String,
//     Hash,
//     List,
//     Set,
//     ZSet,
//     Other,
// }
// // 用于包装错误类型
// type RedisResult<T> = Result<T, Box<dyn Error>>;
// // 定义RedisUtil结构体
// pub struct RedisUtil {
//     pool: Pool<RedisConnectionManager>,
// }
// // // 用于表示有序集合中的元素和分数的元组
// // #[derive(Debug)]
// // struct TypedTuple<T> {
// //     value: T,
// //     score: f64,
// // }

// // impl<T> TypedTuple<T> {
// //     fn new(value: T, score: f64) -> Self {
// //         TypedTuple { value, score }
// //     }
// // }
// /// 建立连接
// impl RedisUtil {
//     // 创建一个新的RedisUtil实例，建立连接池
//     pub async fn new() -> Result<Self, Box<dyn Error>> {
//         let pool = establish_redis_pool_connection().await?;
//         Ok(RedisUtil { pool })
//     }
// }
// /// key相关
// impl RedisUtil {
//     // 设置键值对和过期时间（以秒为单位）
//     pub async fn set_expire(
//         &self,
//         key: &str,
//         value: &str,
//         timeout: u64,
//     ) -> Result<(), Box<dyn Error>> {
//         let mut conn = self.pool.get().await?;
//         let value = conn.set_ex(key, value, timeout).await?;
//         Ok(value)
//     }
//     // 设置键值对
//     pub async fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
//         let mut conn = self.pool.get().await?;
//         let value = conn.set(key, value).await?;
//         Ok(value)
//     }
//     // 设置key的过期时间（以秒为单位）
//     pub async fn expire(&self, key: &str, timeout: u64) -> RedisResult<bool> {
//         let mut conn = self.pool.get().await?;
//         let result: bool = conn.expire(key, timeout.try_into().unwrap()).await?;
//         Ok(result)
//     }

//     // 获取键对应的值
//     pub async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
//         let mut conn = self.pool.get().await?;
//         let value: Option<String> = conn.get(key).await?;
//         Ok(value)
//     }

//     // 删除单个key
//     pub async fn delete(&self, key: &str) -> RedisResult<()> {
//         let mut conn = self.pool.get().await?;
//         let value = conn.del(key).await?;
//         Ok(value)
//     }

//     // 删除多个key
//     pub async fn delete_collection(&self, keys: &[String]) -> RedisResult<()> {
//         let mut conn = self.pool.get().await?;
//         let keys: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
//         let value = conn.del(keys).await?;
//         Ok(value)
//     }

//     // 获取key的序列化值
//     // pub async fn dump(&self, key: &str) -> RedisResult<Vec<u8>> {
//     //     let mut conn = self.pool.get().await?;
//     //     let value: Vec<u8> = conn.dump(key).await?;
//     //     Ok(value)
//     // }

//     // 检查key是否存在
//     pub async fn has_key(&self, key: &str) -> RedisResult<bool> {
//         let mut conn = self.pool.get().await?;
//         let exists: bool = conn.exists(key).await?;
//         Ok(exists)
//     }

//     // 设置key在指定时间点过期
//     pub async fn expire_at(&self, key: &str, date: i64) -> RedisResult<bool> {
//         let mut conn = self.pool.get().await?;
//         let result: bool = conn.expire_at(key, date).await?;
//         Ok(result)
//     }

//     // 获取匹配指定模式的所有key
//     pub async fn keys(&self, pattern: &str) -> RedisResult<HashSet<String>> {
//         let mut conn = self.pool.get().await?;
//         let keys: Vec<String> = conn.keys(pattern).await?;
//         Ok(keys.into_iter().collect())
//     }

//     // 将key移动到指定的数据库
//     // pub async fn move_key(&self, key: &str, db_index: u8) -> RedisResult<bool> {
//     //     let mut conn = self.pool.get().await?;
//     //     let result: bool = conn.move_db(key, db_index).await?;
//     //     Ok(result)
//     // }

//     // 移除key的过期时间
//     pub async fn persist(&self, key: &str) -> RedisResult<bool> {
//         let mut conn = self.pool.get().await?;
//         let result: bool = conn.persist(key).await?;
//         Ok(result)
//     }

//     // 获取key的剩余过期时间（以秒为单位）
//     pub async fn get_expire(&self, key: &str) -> RedisResult<u64> {
//         let mut conn = self.pool.get().await?;
//         let seconds: u64 = conn.ttl(key).await?;
//         Ok(seconds)
//     }

//     // 获取key的剩余过期时间
//     pub async fn get_expire_with_unit(&self, key: &str) -> RedisResult<u64> {
//         let mut conn = self.pool.get().await?;
//         let seconds: u64 = conn.pttl(key).await?;
//         Ok(seconds)
//     }

//     // 获取随机的一个key
//     // pub async fn random_key(&self) -> RedisResult<Option<String>> {
//     //     let mut conn = self.pool.get().await?;
//     //     let key: Option<String> = conn.random_key().await?;
//     //     Ok(key)
//     // }

//     // 重命名key
//     pub async fn rename(&self, old_key: &str, new_key: &str) -> RedisResult<()> {
//         let mut conn = self.pool.get().await?;
//         let value = conn.rename(old_key, new_key).await?;
//         Ok(value)
//     }

//     // 重命名key（如果新key不存在）
//     pub async fn rename_if_absent(&self, old_key: &str, new_key: &str) -> RedisResult<bool> {
//         let mut conn = self.pool.get().await?;
//         let result: bool = conn.rename_nx(old_key, new_key).await?;
//         Ok(result)
//     }
// }
// // /// String相关
// // impl RedisUtil {
// //     // 设置单个key - value对
// //     pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         conn.set(key, value).await?;
// //         Ok(())
// //     }

// //     // 获取key对应的value
// //     pub async fn get(&self, key: &str) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.get(key).await?;
// //         Ok(value)
// //     }

// //     // 获取key对应value的子串
// //     pub async fn get_range(&self, key: &str, start: u64, end: u64) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.getrange(key, start, end).await?;
// //         Ok(value)
// //     }

// //     // 设置key的新值并返回旧值
// //     pub async fn get_and_set(&self, key: &str, value: &str) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let old_value: Option<String> = conn.getset(key, value).await?;
// //         Ok(old_value)
// //     }

// //     // 获取key对应value在指定偏移量处的位值
// //     pub async fn get_bit(&self, key: &str, offset: u64) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let bit: bool = conn.getbit(key, offset).await?;
// //         Ok(bit)
// //     }

// //     // 批量获取多个key对应的value
// //     pub async fn multi_get(&self, keys: &[String]) -> RedisResult<Vec<Option<String>>> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
// //         let values: Vec<Option<String>> = conn.mget(keys).await?;
// //         Ok(values)
// //     }

// //     // 设置key对应value在指定偏移量处的位值
// //     pub async fn set_bit(&self, key: &str, offset: u64, value: bool) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let result: bool = conn.setbit(key, offset, value).await?;
// //         Ok(result)
// //     }

// //     // 设置key - value对并指定过期时间（以秒为单位）
// //     pub async fn set_ex(&self, key: &str, value: &str, timeout: u64) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         conn.set_ex(key, timeout, value).await?;
// //         Ok(())
// //     }

// //     // 如果key不存在则设置key - value对
// //     pub async fn set_if_absent(&self, key: &str, value: &str) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let result: bool = conn.set_nx(key, value).await?;
// //         Ok(result)
// //     }

// //     // 在key对应value的指定偏移量处设置子串
// //     pub async fn set_range(&self, key: &str, value: &str, offset: u64) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let new_len: u64 = conn.setrange(key, offset, value).await?;
// //         Ok(new_len)
// //     }

// //     // 获取key对应value的长度
// //     pub async fn size(&self, key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let len: u64 = conn.strlen(key).await?;
// //         Ok(len)
// //     }

// //     // 批量设置多个key - value对
// //     pub async fn multi_set(&self, maps: HashMap<String, String>) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         let mut pairs: Vec<(&str, &str)> =
// //             maps.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
// //         conn.mset(pairs).await?;
// //         Ok(())
// //     }

// //     // 如果所有key都不存在则批量设置多个key - value对
// //     pub async fn multi_set_if_absent(&self, maps: HashMap<String, String>) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let mut pairs: Vec<(&str, &str)> =
// //             maps.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
// //         let result: bool = conn.msetnx(pairs).await?;
// //         Ok(result)
// //     }

// //     // 将key对应的值增加指定的增量（整数）
// //     pub async fn incr_by(&self, key: &str, increment: i64) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let value: i64 = conn.incr(key, increment).await?;
// //         Ok(value)
// //     }

// //     // 将key对应的值增加指定的增量（浮点数）
// //     pub async fn incr_by_float(&self, key: &str, increment: f64) -> RedisResult<f64> {
// //         let mut conn = self.pool.get().await?;
// //         let value: f64 = conn.incrbyfloat(key, increment).await?;
// //         Ok(value)
// //     }

// //     // 在key对应的值末尾追加字符串
// //     pub async fn append(&self, key: &str, value: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let new_len: u64 = conn.append(key, value).await?;
// //         Ok(new_len)
// //     }
// // }
// // /// hash相关
// // impl RedisUtil {
// //     // 获取hash中指定field的值
// //     pub async fn h_get(&self, key: &str, field: &str) -> RedisResult<Option<Value>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<Value> = conn.hget(key, field).await?;
// //         Ok(value)
// //     }

// //     // 获取hash中的所有键值对
// //     pub async fn h_get_all(&self, key: &str) -> RedisResult<HashMap<String, Value>> {
// //         let mut conn = self.pool.get().await?;
// //         let pairs: HashMap<String, Value> = conn.hgetall(key).await?;
// //         Ok(pairs)
// //     }

// //     // 批量获取hash中多个field的值
// //     pub async fn h_multi_get(
// //         &self,
// //         key: &str,
// //         fields: &[Value],
// //     ) -> RedisResult<Vec<Option<Value>>> {
// //         let mut conn = self.pool.get().await?;
// //         let values: Vec<Option<Value>> = conn.hmget(key, fields).await?;
// //         Ok(values)
// //     }

// //     // 在hash中设置指定field - value对
// //     pub async fn h_put(&self, key: &str, hash_key: &str, value: &str) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         conn.hset(key, hash_key, value).await?;
// //         Ok(())
// //     }

// //     // 批量设置hash中的多个field - value对
// //     pub async fn h_put_all(&self, key: &str, maps: HashMap<String, String>) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         let mut pairs: Vec<(&str, &str)> =
// //             maps.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
// //         conn.hm_set(key, pairs).await?;
// //         Ok(())
// //     }

// //     // 如果hash中指定field不存在则设置field - value对
// //     pub async fn h_put_if_absent(
// //         &self,
// //         key: &str,
// //         hash_key: &str,
// //         value: &str,
// //     ) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let result: bool = conn.hset_nx(key, hash_key, value).await?;
// //         Ok(result)
// //     }

// //     // 在hash中删除指定的field
// //     pub async fn h_delete(&self, key: &str, fields: &[Value]) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.hdel(key, fields).await?;
// //         Ok(count)
// //     }

// //     // 检查hash中是否存在指定的field
// //     pub async fn h_exists(&self, key: &str, field: &str) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let exists: bool = conn.hexists(key, field).await?;
// //         Ok(exists)
// //     }
// //     pub async fn h_incr_by(&self, key: &str, field: &str, increment: i64) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let value: i64 = conn.hincrby(key, field, increment).await?;
// //         Ok(value)
// //     }

// //     // 将hash中指定field的值增加指定的增量（浮点数）
// //     pub async fn h_incr_by_float(&self, key: &str, field: &str, delta: f64) -> RedisResult<f64> {
// //         let mut conn = self.pool.get().await?;
// //         let value: f64 = conn.hincrbyfloat(key, field, delta).await?;
// //         Ok(value)
// //     }

// //     // 获取hash中的所有键
// //     pub async fn h_keys(&self, key: &str) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<String> = conn.hkeys(key).await?;
// //         Ok(keys.into_iter().collect())
// //     }

// //     // 获取hash中的键值对数量
// //     pub async fn h_size(&self, key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let size: u64 = conn.hlen(key).await?;
// //         Ok(size)
// //     }

// //     // 获取hash中的所有值
// //     pub async fn h_values(&self, key: &str) -> RedisResult<Vec<Value>> {
// //         let mut conn = self.pool.get().await?;
// //         let values: Vec<Value> = conn.hvals(key).await?;
// //         Ok(values)
// //     }

// //     // 对hash进行扫描操作
// //     pub async fn h_scan(
// //         &self,
// //         key: &str,
// //         options: ScanArgs,
// //     ) -> RedisResult<(HashSet<String>, ScanCursor)> {
// //         let mut conn = self.pool.get().await?;
// //         let (keys, cursor): (Vec<String>, ScanCursor) = conn.hscan(key, options).await?;
// //         Ok((keys.into_iter().collect(), cursor))
// //     }
// // }
// // /// list相关
// // impl RedisUtil {
// //     // 获取列表中指定索引位置的元素
// //     pub async fn l_index(&self, key: &str, index: i64) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.lindex(key, index).await?;
// //         Ok(value)
// //     }

// //     // 获取列表中指定范围的元素
// //     pub async fn l_range(&self, key: &str, start: i64, end: i64) -> RedisResult<Vec<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let values: Vec<String> = conn.lrange(key, start, end).await?;
// //         Ok(values)
// //     }

// //     // 在列表头部插入一个元素
// //     pub async fn l_left_push(&self, key: &str, value: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.lpush(key, value).await?;
// //         Ok(length)
// //     }

// //     // 在列表头部插入多个元素
// //     pub async fn l_left_push_all(&self, key: &str, values: &[String]) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let mut value_slice: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
// //         let length: i64 = conn.lpush(key, &value_slice).await?;
// //         Ok(length)
// //     }

// //     // 如果列表存在，则在列表头部插入一个元素
// //     pub async fn l_left_push_if_present(&self, key: &str, value: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.lpushx(key, value).await?;
// //         Ok(length)
// //     }

// //     // 在列表中指定元素之前插入一个元素
// //     pub async fn l_left_push_pivot(&self, key: &str, pivot: &str, value: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.linsert(key, false, pivot, value).await?;
// //         Ok(length)
// //     }

// //     // 在列表尾部插入一个元素
// //     pub async fn l_right_push(&self, key: &str, value: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.rpush(key, value).await?;
// //         Ok(length)
// //     }

// //     // 在列表尾部插入多个元素
// //     pub async fn l_right_push_all(&self, key: &str, values: &[String]) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let mut value_slice: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
// //         let length: i64 = conn.rpush(key, &value_slice).await?;
// //         Ok(length)
// //     }

// //     // 如果列表存在，则在列表尾部插入一个元素
// //     pub async fn l_right_push_if_present(&self, key: &str, value: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.rpushx(key, value).await?;
// //         Ok(length)
// //     }

// //     // 在列表中指定元素之后插入一个元素
// //     pub async fn l_right_push_pivot(
// //         &self,
// //         key: &str,
// //         pivot: &str,
// //         value: &str,
// //     ) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.linsert(key, true, pivot, value).await?;
// //         Ok(length)
// //     }

// //     // 设置列表中指定索引位置的元素
// //     pub async fn l_set(&self, key: &str, index: i64, value: &str) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         conn.lset(key, index, value).await?;
// //         Ok(())
// //     }

// //     // 从列表头部弹出一个元素
// //     pub async fn l_left_pop(&self, key: &str) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.lpop(key).await?;
// //         Ok(value)
// //     }

// //     // 阻塞式从列表头部弹出一个元素（设置超时时间）
// //     pub async fn l_b_left_pop(&self, key: &str, timeout: Duration) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.blpop(key, timeout).await?;
// //         Ok(value)
// //     }

// //     // 从列表尾部弹出一个元素
// //     pub async fn l_right_pop(&self, key: &str) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.rpop(key).await?;
// //         Ok(value)
// //     }

// //     // 阻塞式从列表尾部弹出一个元素（设置超时时间）
// //     pub async fn l_b_right_pop(&self, key: &str, timeout: Duration) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.brpop(key, timeout).await?;
// //         Ok(value)
// //     }

// //     // 从一个列表尾部弹出一个元素并插入到另一个列表头部
// //     pub async fn l_right_pop_and_left_push(
// //         &self,
// //         source_key: &str,
// //         destination_key: &str,
// //     ) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.rpoplpush(source_key, destination_key).await?;
// //         Ok(value)
// //     }

// //     // 阻塞式从一个列表尾部弹出一个元素并插入到另一个列表头部（设置超时时间）
// //     pub async fn l_b_right_pop_and_left_push(
// //         &self,
// //         source_key: &str,
// //         destination_key: &str,
// //         timeout: Duration,
// //     ) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn
// //             .brpoplpush(source_key, destination_key, timeout)
// //             .await?;
// //         Ok(value)
// //     }

// //     // 从列表中删除指定数量的指定元素
// //     pub async fn l_remove(&self, key: &str, count: i64, value: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let removed_count: i64 = conn.lrem(key, count, value).await?;
// //         Ok(removed_count)
// //     }

// //     // 修剪列表，保留指定范围的元素
// //     pub async fn l_trim(&self, key: &str, start: i64, end: i64) -> RedisResult<()> {
// //         let mut conn = self.pool.get().await?;
// //         conn.ltrim(key, start, end).await?;
// //         Ok(())
// //     }

// //     // 获取列表的长度
// //     pub async fn l_len(&self, key: &str) -> RedisResult<i64> {
// //         let mut conn = self.pool.get().await?;
// //         let length: i64 = conn.llen(key).await?;
// //         Ok(length)
// //     }
// // }
// // /// set相关
// // impl RedisUtil {
// //     // 向集合中添加一个或多个元素
// //     pub async fn s_add(&self, key: &str, values: &[String]) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let mut value_slice: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
// //         let added_count: u64 = conn.sadd(key, &value_slice).await?;
// //         Ok(added_count)
// //     }

// //     // 从集合中删除一个或多个元素
// //     pub async fn s_remove(&self, key: &str, values: &[Value]) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let removed_count: u64 = conn.srem(key, values).await?;
// //         Ok(removed_count)
// //     }

// //     // 从集合中随机弹出一个元素
// //     pub async fn s_pop(&self, key: &str) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let value: Option<String> = conn.spop(key).await?;
// //         Ok(value)
// //     }

// //     // 将集合中的一个元素移动到另一个集合
// //     pub async fn s_move(&self, key: &str, value: &str, dest_key: &str) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let moved: bool = conn.smove(key, value, dest_key).await?;
// //         Ok(moved)
// //     }

// //     // 获取集合的元素数量
// //     pub async fn s_size(&self, key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let size: u64 = conn.scard(key).await?;
// //         Ok(size)
// //     }

// //     // 检查元素是否是集合的成员
// //     pub async fn s_is_member(&self, key: &str, value: &Value) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let is_member: bool = conn.sismember(key, value).await?;
// //         Ok(is_member)
// //     }

// //     // 获取两个集合的交集
// //     pub async fn s_intersect(&self, key: &str, other_key: &str) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.sinter(key, &[other_key]).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取多个集合的交集
// //     pub async fn s_intersect_collection(
// //         &self,
// //         key: &str,
// //         other_keys: &[String],
// //     ) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let members: Vec<String> = conn.sinter(key, &keys).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取两个集合的交集并存储到目标集合
// //     pub async fn s_intersect_and_store(
// //         &self,
// //         key: &str,
// //         other_key: &str,
// //         dest_key: &str,
// //     ) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.sinterstore(dest_key, &[key, other_key]).await?;
// //         Ok(count)
// //     }

// //     // 获取多个集合的交集并存储到目标集合
// //     pub async fn s_intersect_and_store_collection(
// //         &self,
// //         key: &str,
// //         other_keys: &[String],
// //         dest_key: &str,
// //     ) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let count: u64 = conn
// //             .sinterstore(dest_key, &[key].iter().chain(keys).collect::<Vec<&str>>())
// //             .await?;
// //         Ok(count)
// //     }

// //     // 获取两个集合的并集
// //     pub async fn s_union(&self, key: &str, other_key: &str) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.sunion(key, &[other_key]).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取多个集合的并集
// //     pub async fn s_union_collection(
// //         &self,
// //         key: &str,
// //         other_keys: &[String],
// //     ) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let members: Vec<String> = conn.sunion(key, &keys).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取两个集合的并集并存储到目标集合
// //     pub async fn s_union_and_store(
// //         &self,
// //         key: &str,
// //         other_key: &str,
// //         dest_key: &str,
// //     ) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.sunionstore(dest_key, &[key, other_key]).await?;
// //         Ok(count)
// //     }

// //     // 获取多个集合的并集并存储到目标集合
// //     pub async fn s_union_and_store_collection(
// //         &self,
// //         key: &str,
// //         other_keys: &[String],
// //         dest_key: &str,
// //     ) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let count: u64 = conn
// //             .sunionstore(dest_key, &[key].iter().chain(keys).collect::<Vec<&str>>())
// //             .await?;
// //         Ok(count)
// //     }

// //     // 获取两个集合的差集
// //     pub async fn s_difference(&self, key: &str, other_key: &str) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.sdiff(key, &[other_key]).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取多个集合的差集
// //     pub async fn s_difference_collection(
// //         &self,
// //         key: &str,
// //         other_keys: &[String],
// //     ) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let members: Vec<String> = conn.sdiff(key, &keys).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取两个集合的差集并存储到目标集合
// //     pub async fn s_difference_and_store(
// //         &self,
// //         key: &str,
// //         other_key: &str,
// //         dest_key: &str,
// //     ) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.sdiffstore(dest_key, &[key, other_key]).await?;
// //         Ok(count)
// //     }
// //     // 获取多个集合的差集并存储到目标集合
// //     pub async fn s_difference_and_store_collection(
// //         &self,
// //         key: &str,
// //         other_keys: &[String],
// //         dest_key: &str,
// //     ) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let count: u64 = conn
// //             .sdiffstore(dest_key, &[key].iter().chain(keys).collect::<Vec<&str>>())
// //             .await?;
// //         Ok(count)
// //     }

// //     // 获取集合中的所有元素
// //     pub async fn set_members(&self, key: &str) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.smembers(key).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 从集合中随机获取一个元素
// //     pub async fn s_random_member(&self, key: &str) -> RedisResult<Option<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let member: Option<String> = conn.srandmember(key).await?;
// //         Ok(member)
// //     }

// //     // 从集合中随机获取指定数量的元素
// //     pub async fn s_random_members(&self, key: &str, count: u64) -> RedisResult<Vec<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.srandmember(key, count).await?;
// //         Ok(members)
// //     }

// //     // 从集合中随机获取指定数量的不重复元素
// //     pub async fn s_distinct_random_members(
// //         &self,
// //         key: &str,
// //         count: u64,
// //     ) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.srandmember(key, count).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 对集合进行扫描操作
// //     pub async fn s_scan(
// //         &self,
// //         key: &str,
// //         options: ScanArgs,
// //     ) -> RedisResult<(HashSet<String>, ScanCursor)> {
// //         let mut conn = self.pool.get().await?;
// //         let (keys, cursor): (Vec<String>, ScanCursor) = conn.sscan(key, options).await?;
// //         Ok((keys.into_iter().collect(), cursor))
// //     }
// // }
// // /// zSet相关
// // impl RedisUtil {
// //     // 向有序集合中添加一个元素（指定分数）
// //     pub async fn z_add(&self, key: &str, value: &str, score: f64) -> RedisResult<bool> {
// //         let mut conn = self.pool.get().await?;
// //         let added: bool = conn.zadd(key, &[(value, score)], None).await?;
// //         Ok(added)
// //     }

// //     // 向有序集合中添加多个元素（指定分数）
// //     pub async fn z_add_multiple(&self, key: &str, values: &[(String, f64)]) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let mut value_tuples: Vec<(RedisString, f64)> = values
// //            .iter()
// //            .map(|(s, score)| (s.as_str().into(), *score))
// //            .collect();
// //         let added_count: u64 = conn.zadd(key, &value_tuples, None).await?;
// //         Ok(added_count)
// //     }

// //     // 从有序集合中删除一个或多个元素
// //     pub async fn z_remove(&self, key: &str, values: &[Value]) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let removed_count: u64 = conn.zrem(key, values).await?;
// //         Ok(removed_count)
// //     }

// //     // 增加有序集合中指定元素的分数
// //     pub async fn z_increment_score(&self, key: &str, value: &str, delta: f64) -> RedisResult<f64> {
// //         let mut conn = self.pool.get().await?;
// //         let new_score: f64 = conn.zincrby(key, delta, value).await?;
// //         Ok(new_score)
// //     }

// //     // 获取有序集合中指定元素的排名（从小到大）
// //     pub async fn z_rank(&self, key: &str, value: &Value) -> RedisResult<Option<i64>> {
// //         let mut conn = self.pool.get().await?;
// //         let rank: Option<i64> = conn.zrank(key, value).await?;
// //         Ok(rank)
// //     }

// //     // 获取有序集合中指定元素的排名（从大到小）
// //     pub async fn z_reverse_rank(&self, key: &str, value: &Value) -> RedisResult<Option<i64>> {
// //         let mut conn = self.pool.get().await?;
// //         let rank: Option<i64> = conn.zrevrank(key, value).await?;
// //         Ok(rank)
// //     }

// //     // 获取有序集合中指定排名范围的元素（从小到大）
// //     pub async fn z_range(&self, key: &str, start: i64, end: i64) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.zrange(key, start, end).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取有序集合中指定排名范围的元素（包含分数，从小到大）
// //     pub async fn z_range_with_scores(&self, key: &str, start: i64, end: i64) -> RedisResult<Vec<(String, f64)>> {
// //         let mut conn = self.pool.get().await?;
// //         let tuples: Vec<(RedisString, f64)> = conn.zrange_with_scores(key, start, end).await?;
// //         let pairs: Vec<(String, f64)> = tuples
// //            .iter()
// //            .map(|(s, score)| (s.to_string(), *score))
// //            .collect();
// //         Ok(pairs)
// //     }

// //     // 获取有序集合中指定分数范围的元素（从小到大）
// //     pub async fn z_range_by_score(&self, key: &str, min: f64, max: f64) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.zrangebyscore(key, min, max).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取有序集合中指定分数范围的元素（包含分数，从小到大）
// //     pub async fn z_range_by_score_with_scores(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<(String, f64)>> {
// //         let mut conn = self.pool.get().await?;
// //         let tuples: Vec<(RedisString, f64)> = conn.zrangebyscore_with_scores(key, min, max).await?;
// //         let pairs: Vec<(String, f64)> = tuples
// //            .iter()
// //            .map(|(s, score)| (s.to_string(), *score))
// //            .collect();
// //         Ok(pairs)
// //     }

// //     // 获取有序集合中指定分数范围和排名范围的元素（包含分数，从小到大）
// //     pub async fn z_range_by_score_with_scores_range(&self, key: &str, min: f64, max: f64, start: i64, end: i64) -> RedisResult<Vec<(String, f64)>> {
// //         let mut conn = self.pool.get().await?;
// //         let tuples: Vec<(RedisString, f64)> = conn.zrangebyscore_with_scores(key, min, max, start, end).await?;
// //         let pairs: Vec<(String, f64)> = tuples
// //            .iter()
// //            .map(|(s, score)| (s.to_string(), *score))
// //            .collect();
// //         Ok(pairs)
// //     }

// //     // 获取有序集合中指定排名范围的元素（从大到小）
// //     pub async fn z_reverse_range(&self, key: &str, start: i64, end: i64) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.zrevrange(key, start, end).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取有序集合中指定排名范围的元素（包含分数，从大到小）
// //     pub async fn z_reverse_range_with_scores(&self, key: &str, start: i64, end: i64) -> RedisResult<Vec<(String, f64)>> {
// //         let mut conn = self.pool.get().await?;
// //         let tuples: Vec<(RedisString, f64)> = conn.zrevrange_with_scores(key, start, end).await?;
// //         let pairs: Vec<(String, f64)> = tuples
// //            .iter()
// //            .map(|(s, score)| (s.to_string(), *score))
// //            .collect();
// //         Ok(pairs)
// //     }

// //     // 获取有序集合中指定分数范围的元素（从大到小）
// //     pub async fn z_reverse_range_by_score(&self, key: &str, min: f64, max: f64) -> RedisResult<HashSet<String>> {
// //         let mut conn = self.pool.get().await?;
// //         let members: Vec<String> = conn.zrevrangebyscore(key, min, max).await?;
// //         Ok(members.into_iter().collect())
// //     }

// //     // 获取有序集合中指定分数范围的元素（包含分数，从大到小）
// //     pub async fn z_reverse_range_by_score_with_scores(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<(String, f64)>> {
// //         let mut conn = self.pool.get().await?;
// //         let tuples: Vec<(RedisString, f64)> = conn.zrevrangebyscore_with_scores(key, min, max).await?;
// //         let pairs: Vec<(String, f64)> = tuples
// //            .iter()
// //            .map(|(s, score)| (s.to_string(), *score))
// //            .collect();
// //         Ok(pairs)
// //     }

// //     // 获取有序集合中指定分数范围和排名范围的元素（包含分数，从大到小）
// //     pub async fn z_reverse_range_by_score_with_scores_range(&self, key: &str, min: f64, max: f64, start: i64, end: i64) -> RedisResult<Vec<(String, f64)>> {
// //         let mut conn = self.pool.get().await?;
// //         let tuples: Vec<(RedisString, f64)> = conn.zrevrangebyscore_with_scores(key, min, max, start, end).await?;
// //         let pairs: Vec<(String, f64)> = tuples
// //            .iter()
// //            .map(|(s, score)| (s.to_string(), *score))
// //            .collect();
// //         Ok(pairs)
// //     }

// //     // 计算有序集合中指定分数范围的元素数量
// //     pub async fn z_count(&self, key: &str, min: f64, max: f64) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.zcount(key, min, max).await?;
// //         Ok(count)
// //     }

// //     // 获取有序集合的元素数量
// //     pub async fn z_size(&self, key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let size: u64 = conn.zcard(key).await?;
// //         Ok(size)
// //     }

// //     // 获取有序集合中指定元素的分数
// //     pub async fn z_score(&self, key: &str, value: &Value) -> RedisResult<Option<f64>> {
// //         let mut conn = self.pool.get().await?;
// //         let score: Option<f64> = conn.zscore(key, value).await?;
// //         Ok(score)
// //     }

// //     // 删除有序集合中指定排名范围的元素
// //     pub async fn z_remove_range(&self, key: &str, start: i64, end: i64) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let removed_count: u64 = conn.zremrangebyrank(key, start, end).await?;
// //         Ok(removed_count)
// //     }

// //     // 删除有序集合中指定分数范围的元素
// //     pub async fn z_remove_range_by_score(&self, key: &str, min: f64, max: f64) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let removed_count: u64 = conn.zremrangebyscore(key, min, max).await?;
// //         Ok(removed_count)
// //     }

// //     // 计算多个有序集合的并集并存储到目标集合
// //     pub async fn z_union_and_store(&self, key: &str, other_key: &str, dest_key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.zunionstore(dest_key, &[key, other_key], None).await?;
// //         Ok(count)
// //     }

// //     // 计算多个有序集合的并集并存储到目标集合
// //     pub async fn z_union_and_store_collection(&self, key: &str, other_keys: &[String], dest_key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let count: u64 = conn.zunionstore(dest_key, &[key].iter().chain(keys).collect::<Vec<&str>>(), None).await?;
// //         Ok(count)
// //     }

// //     // 计算多个有序集合的交集并存储到目标集合
// //     pub async fn z_intersect_and_store(&self, key: &str, other_key: &str, dest_key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let count: u64 = conn.zinterstore(dest_key, &[key, other_key], None).await?;
// //         Ok(count)
// //     }

// //     // 计算多个有序集合的交集并存储到目标集合
// //     pub async fn z_intersect_and_store_collection(&self, key: &str, other_keys: &[String], dest_key: &str) -> RedisResult<u64> {
// //         let mut conn = self.pool.get().await?;
// //         let keys: Vec<&str> = other_keys.iter().map(|s| s.as_str()).collect();
// //         let count: u64 = conn.zinterstore(dest_key, &[key].iter().chain(keys).collect::<Vec<&str>>(), None).await?;
// //         Ok(count)
// //     }

// //     // 对有序集合进行扫描操作
// //     pub async fn z_scan(&self, key: &str, options: ScanArgs) -> RedisResult<(HashSet<String>, ScanCursor)> {
// //         let mut conn = self.pool.get().await?;
// //         let (keys, cursor): (Vec<String>, ScanCursor) = conn.zscan(key, options).await?;
// //         Ok((keys.into_iter().collect(), cursor))
// //     }
// // }
