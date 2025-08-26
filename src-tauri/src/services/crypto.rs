use sha2::{Digest, Sha256};
use crate::error::{AppError, AppResult};
use rand::RngCore;

pub struct CryptoService {
    is_authenticated: bool,
}

impl CryptoService {
    pub fn new() -> Self {
        Self { 
            is_authenticated: false 
        }
    }
    
    // 生成密码哈希
    pub fn hash_password(password: &str, salt: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        format!("{:x}", hasher.finalize())
    }
    
    // 设置主密钥（简化版）
    pub fn set_master_key(&mut self, _password: &str, _salt: &[u8]) {
        self.is_authenticated = true;
    }
    
    // 加密数据（模拟）
    pub fn encrypt(&self, data: &[u8]) -> AppResult<EncryptedData> {
        if !self.is_authenticated {
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
        if !self.is_authenticated {
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