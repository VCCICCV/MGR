use std::{ fs, path::PathBuf };
use serde::Deserialize;

use crate::utils::dir;

// 密钥配置
#[derive(Debug, Deserialize, Clone)]
pub struct SecretConfig {
    // 私钥
    pub private_access_key: PathBuf,
    // 公钥
    pub public_access_key: PathBuf,
    // 刷新私钥
    pub private_refresh_key: PathBuf,
    // 刷新公钥
    pub public_refresh_key: PathBuf,
}

impl SecretConfig {
    // 读取私钥
    pub fn read_private_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(dir::get_project_root()?.join(&self.private_access_key))
    }
    // 读取公钥
    pub fn read_public_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(dir::get_project_root()?.join(&self.public_access_key))
    }
    // 读取私钥
    pub fn read_private_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(dir::get_project_root()?.join(&self.private_refresh_key))
    }
    // 读取公钥
    pub fn read_public_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(dir::get_project_root()?.join(&self.public_refresh_key))
    }
}

#[cfg(test)]
mod tests {
    use crate::constant::CONFIG;

    #[test]
    fn test_read_private_access_key() {
        let key = CONFIG.secret.read_private_access_key().unwrap();
        assert!(!key.is_empty())
    }

    #[test]
    fn test_read_public_access_key() {
        let key = CONFIG.secret.read_public_access_key().unwrap();
        assert!(!key.is_empty())
    }

    #[test]
    fn test_read_private_refresh_key() {
        let key = CONFIG.secret.read_private_refresh_key().unwrap();
        assert!(!key.is_empty())
    }

    #[test]
    fn test_read_public_refresh_key() {
        let key = CONFIG.secret.read_public_refresh_key().unwrap();
        assert!(!key.is_empty())
    }
}
