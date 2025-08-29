use sha2::{Digest, Sha256};
use crate::error::{AppError, AppResult};
use rand::RngCore;

pub struct CryptoService {
    master_key_hash: Option<String>,
    salt: Option<[u8; 32]>,
}

impl CryptoService {
    pub fn new() -> Self {
        Self { 
            master_key_hash: None,
            salt: None,
        }
    }
    
    // 生成密码哈希
    pub fn hash_password(password: &str, salt: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        format!("{:x}", hasher.finalize())
    }
    
    // 设置主密码哈希
    pub fn set_master_key_hash(&mut self, hash: String) {
        self.master_key_hash = Some(hash);
    }
    
    // 设置盐值
    pub fn set_salt(&mut self, salt: [u8; 32]) {
        self.salt = Some(salt);
    }
    
    // 清除主密钥
    pub fn clear_master_key(&mut self) {
        self.master_key_hash = None;
        self.salt = None;
    }

    // 获取主密码哈希
    pub fn get_master_key_hash(&self) -> Option<String> {
        self.master_key_hash.clone()
    }

    // 验证密码
    pub fn verify_password(&self, password: &str) -> bool {
        if let (Some(hash), Some(salt)) = (&self.master_key_hash, &self.salt) {
            let input_hash = Self::hash_password(password, salt);
            &input_hash == hash
        } else {
            false
        }
    }
    
    // 检查是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.master_key_hash.is_some()
    }
    
    // 加密数据（模拟）
    pub fn encrypt(&self, data: &[u8]) -> AppResult<EncryptedData> {
        if !self.is_authenticated() {
            return Err(AppError::Unknown("主密钥未设置".to_string()));
        }
        
        // 简单的 XOR "加密"（仅作演示）
        let key_byte = 0x42; // 简单的密钥
        let encrypted: Vec<u8> = data.iter().map(|b| b ^ key_byte).collect();
        
        Ok(EncryptedData {
            nonce: vec![0u8; 12], // 模拟 nonce
            ciphertext: encrypted,
        })
    }
    
    // 解密数据（模拟）
    pub fn decrypt(&self, encrypted: &EncryptedData) -> AppResult<Vec<u8>> {
        if !self.is_authenticated() {
            return Err(AppError::Unknown("主密钥未设置".to_string()));
        }
        
        // 简单的 XOR "解密"
        let key_byte = 0x42;
        let decrypted: Vec<u8> = encrypted.ciphertext.iter().map(|b| b ^ key_byte).collect();
        
        Ok(decrypted)
    }
    
    // 生成随机盐值
    pub fn generate_salt() -> [u8; 32] {
        let mut salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}