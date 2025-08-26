use std::path::PathBuf;
use crate::services::crypto::EncryptedData;
use crate::error::{AppError, AppResult};

pub struct StorageService {
    storage_path: PathBuf,
}

impl StorageService {
    pub fn new() -> AppResult<Self> {
        let app_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("无法获取配置目录".to_string()))?
            .join("sshmanager");
        
        // 确保目录存在
        std::fs::create_dir_all(&app_dir)?;
        
        let storage_path = app_dir.join("storage.enc");
        
        Ok(Self { storage_path })
    }
    
    pub fn is_initialized(&self) -> bool {
        self.storage_path.exists()
    }
    
    pub fn save_encrypted_data(&mut self, encrypted_data: &EncryptedData, salt: &[u8]) -> AppResult<()> {
        let storage_data = crate::types::EncryptedStorage {
            version: "1.0".to_string(),
            salt: salt.to_vec(),
            iv: encrypted_data.nonce.clone(),
            encrypted_data: encrypted_data.ciphertext.clone(),
            checksum: self.calculate_checksum(&encrypted_data.ciphertext),
        };
        
        let serialized = serde_json::to_string(&storage_data)?;
        std::fs::write(&self.storage_path, serialized)?;
        
        Ok(())
    }
    
    pub fn load_encrypted_data(&self) -> AppResult<(EncryptedData, Vec<u8>)> {
        let content = std::fs::read_to_string(&self.storage_path)?;
        let storage_data: crate::types::EncryptedStorage = serde_json::from_str(&content)?;
        
        // 验证校验和
        let calculated_checksum = self.calculate_checksum(&storage_data.encrypted_data);
        if calculated_checksum != storage_data.checksum {
            return Err(AppError::ConfigError("数据完整性验证失败".to_string()));
        }
        
        let encrypted_data = EncryptedData {
            nonce: storage_data.iv,
            ciphertext: storage_data.encrypted_data,
        };
        
        Ok((encrypted_data, storage_data.salt))
    }
    
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}