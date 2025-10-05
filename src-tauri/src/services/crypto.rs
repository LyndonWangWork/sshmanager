use crate::error::{AppError, AppResult};
use crate::utils::constant_time_eq;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Digest;
use zeroize::{ZeroizeOnDrop, Zeroizing};

#[derive(ZeroizeOnDrop)]
pub struct CryptoService {
    master_key: Option<Zeroizing<[u8; 32]>>,
    master_key_hash: Option<String>,
    salt: Option<[u8; 32]>,
}

// 为CryptoService添加公共方法来访问salt
impl CryptoService {
    pub fn new() -> Self {
        Self {
            master_key: None,
            master_key_hash: None,
            salt: None,
        }
    }

    // 使用PBKDF2派生密钥
    pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<sha2::Sha256>(password.as_bytes(), salt, 100_000, &mut key);
        key
    }

    // 生成密码哈希（用于向后兼容）
    pub fn hash_password(password: &str, salt: &[u8]) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        format!("{:x}", hasher.finalize())
    }

    // 设置主密码
    pub fn set_master_key(&mut self, password: &str) -> AppResult<()> {
        let salt = Self::generate_salt();
        let derived_key = Self::derive_key(password, &salt);

        // 使用Zeroizing包装器确保密钥在作用域结束时自动清零
        self.master_key = Some(Zeroizing::new(derived_key));
        self.salt = Some(salt);

        // 存储派生密钥的哈希用于后续验证
        let key_hash = Self::hash_key(&derived_key, &salt);
        self.master_key_hash = Some(key_hash);

        Ok(())
    }

    // 设置主密码哈希
    pub fn set_master_key_hash(&mut self, hash: String) {
        self.master_key_hash = Some(hash);
    }

    // 设置盐值
    pub fn set_salt(&mut self, salt: [u8; 32]) {
        self.salt = Some(salt);
    }

    // 直接设置派生的主密钥（用于认证后设置）
    pub fn set_derived_master_key(&mut self, derived_key: [u8; 32]) {
        self.master_key = Some(Zeroizing::new(derived_key));
    }

    // 清除主密钥
    pub fn clear_master_key(&mut self) {
        self.master_key = None;
        self.master_key_hash = None;
        self.salt = None;
    }

    // 获取主密码哈希
    pub fn get_master_key_hash(&self) -> Option<String> {
        self.master_key_hash.clone()
    }

    // 获取盐值
    pub fn get_salt(&self) -> Option<[u8; 32]> {
        self.salt
    }

    // 验证密码
    pub fn verify_password(&self, password: &str) -> bool {
        if let (Some(stored_hash), Some(salt)) = (&self.master_key_hash, &self.salt) {
            let derived_key = Self::derive_key(password, salt);
            let derived_hash = Self::hash_key(&derived_key, salt);
            // 使用常量时间比较防止时序攻击
            constant_time_eq(stored_hash.as_bytes(), derived_hash.as_bytes())
        } else {
            false
        }
    }

    // 检查是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.master_key.is_some()
    }

    // AES-256-GCM加密
    pub fn encrypt(&self, data: &[u8]) -> AppResult<EncryptedData> {
        if !self.is_authenticated() {
            return Err(AppError::Unknown("主密钥未设置".to_string()));
        }

        let key = self.master_key.as_ref().unwrap();
        let cipher = Aes256Gcm::new(key.as_ref().into());
        let mut nonce_bytes = [0u8; 12]; // 96位nonce
        let mut rng = OsRng;
        rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|_| AppError::Unknown("加密失败".to_string()))?;

        Ok(EncryptedData {
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        })
    }

    // AES-256-GCM解密
    pub fn decrypt(&self, encrypted: &EncryptedData) -> AppResult<Vec<u8>> {
        if !self.is_authenticated() {
            return Err(AppError::Unknown("主密钥未设置".to_string()));
        }

        let key = self.master_key.as_ref().unwrap();
        let cipher = Aes256Gcm::new(key.as_ref().into());
        let nonce = Nonce::from_slice(&encrypted.nonce);

        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|_| AppError::Unknown("解密失败".to_string()))?;

        Ok(plaintext)
    }

    // 生成随机盐值
    pub fn generate_salt() -> [u8; 32] {
        let mut salt = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut salt);
        salt
    }

    // 哈希密钥
    fn hash_key(key: &[u8], salt: &[u8]) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(key);
        hasher.update(salt);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}
