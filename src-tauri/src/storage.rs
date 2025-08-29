use std::path::PathBuf;
use crate::services::crypto::EncryptedData;
use crate::error::{AppError, AppResult};

pub struct StorageService {
    storage_path: PathBuf,
}

impl StorageService {

    //
    pub fn new() -> AppResult<Self> {
        let app_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("无法获取配置目录".to_string()))?
            .join("sshmanager");
        
        // 确保目录存在
        std::fs::create_dir_all(&app_dir)?;
        
        let storage_path = app_dir.join("storage.enc");
        
        Ok(Self { storage_path })
    }
    
    // 用于测试的构造函数
    #[cfg(test)]
    pub fn new_for_test(storage_path: PathBuf) -> Self {
        Self { storage_path }
    }
    
    // 返回storage_path
    pub fn storage_path(&self) -> &PathBuf {
        &self.storage_path
    }

    
    pub fn is_initialized(&self) -> bool {
        if !self.storage_path.exists() {
            return false;
        }
        
        // 读取文件内容
        match std::fs::read_to_string(&self.storage_path) {
            Ok(content) => {
                // 解析 JSON 数据
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(data) => {
                        // 检查顶层字段中是否存在主密钥哈希字段,且主密钥哈希字段不为空
                        if let Some(master_key_hash) = data.get("master_key_hash") {
                            if let Some(hash_str) = master_key_hash.as_str() {
                                return !hash_str.is_empty();
                            }
                        }
                        false
                    },
                    Err(_) => false
                }
            },
            Err(_) => false
        }
    }

    pub fn save_encrypted_data(&mut self, encrypted_data: &EncryptedData, salt: &[u8], master_key_hash: &str) -> AppResult<()> {
        // 创建存储数据
        let mut data_map = serde_json::Map::new();
        // 注意：不在data_map中存储salt和master_key_hash，因为它们已经是EncryptedStorage结构体的顶层字段
        
        // 创建空的keys数组
        data_map.insert("keys".to_string(), serde_json::Value::Array(vec![]));
        
        // 创建配置对象
        let mut config_map = serde_json::Map::new();
        config_map.insert("theme".to_string(), serde_json::Value::String("light".to_string()));
        config_map.insert("auto_backup".to_string(), serde_json::Value::Bool(true));
        config_map.insert("backup_retention".to_string(), serde_json::Value::Number(10.into()));
        config_map.insert("default_key_type".to_string(), serde_json::Value::String("Ed25519".to_string()));
        config_map.insert("default_key_size".to_string(), serde_json::Value::Number(256.into()));
        
        data_map.insert("config".to_string(), serde_json::Value::Object(config_map));
        
        let storage_data = crate::types::EncryptedStorage {
            version: "1.0".to_string(),
            salt: salt.to_vec(),
            master_key_hash: master_key_hash.to_string(),
            iv: encrypted_data.nonce.clone(),
            encrypted_data: encrypted_data.ciphertext.clone(),
            checksum: self.calculate_checksum(&encrypted_data.ciphertext),
            data: data_map,
        };
        
        let serialized = serde_json::to_string(&storage_data)?;
        std::fs::write(&self.storage_path, serialized)?;
        
        Ok(())
    }
    
    pub fn load_encrypted_data(&self) -> AppResult<(EncryptedData, Vec<u8>, String, [u8; 32])> {
        let content = std::fs::read_to_string(&self.storage_path)?;
        let storage_data: crate::types::EncryptedStorage = serde_json::from_str(&content)?;
        
        // 验证校验和
        let calculated_checksum = self.calculate_checksum(&storage_data.encrypted_data);
        if calculated_checksum != storage_data.checksum {
            return Err(AppError::ConfigError("数据完整性验证失败".to_string()));
        }
        
        // 从顶层字段解析主密码哈希和盐值，而不是从data字段
        let master_key_hash = storage_data.master_key_hash;
        
        // 直接使用顶层的salt字段
        let salt_vec = storage_data.salt.clone();

        // 确保盐值长度为32字节
        if salt_vec.len() != 32 {
            return Err(AppError::ConfigError("盐值长度不正确".to_string()));
        }
        
        let mut salt = [0u8; 32];
        salt.copy_from_slice(&salt_vec);
        
        // 返回加密数据、盐值、主密码哈希
        Ok((
            EncryptedData {
                nonce: storage_data.iv,
                ciphertext: storage_data.encrypted_data,
            },
            salt_vec,
            master_key_hash,
            salt,
        ))
    }
    
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    pub fn reset_storage(&mut self) -> AppResult<()> {
        // 如果文件存在，则删除它
        if self.storage_path.exists() {
            std::fs::remove_file(&self.storage_path)?;
        }
        
        // 重新初始化存储
        let app_dir = self.storage_path.parent()
            .ok_or_else(|| AppError::ConfigError("无法获取存储目录".to_string()))?;
        
        // 确保目录存在（这会创建目录如果它不存在）
        std::fs::create_dir_all(app_dir)?;
        
        // 创建新的空存储文件
        std::fs::File::create(&self.storage_path)?;
        
        Ok(())
    }
}