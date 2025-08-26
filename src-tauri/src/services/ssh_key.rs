use uuid::Uuid;
use chrono::Utc;
use sha2::{Digest, Sha256};
use crate::types::{SshKeyPair, SshKeyType, KeyGenerationParams};
use crate::error::{AppError, AppResult};

pub struct SshKeyService;

impl SshKeyService {
    // 生成SSH密钥对（使用模拟数据，后续可以改为真实生成）
    pub fn generate_key_pair(params: KeyGenerationParams) -> AppResult<SshKeyPair> {
        let (private_key, public_key) = match params.key_type {
            SshKeyType::Rsa => Self::generate_mock_rsa_key(params.key_size)?,
            SshKeyType::Ed25519 => Self::generate_mock_ed25519_key()?,
            SshKeyType::Ecdsa => Self::generate_mock_ecdsa_key(params.key_size)?,
        };
        
        let fingerprint = Self::calculate_fingerprint(&public_key)?;
        
        Ok(SshKeyPair {
            id: Uuid::new_v4().to_string(),
            name: params.name,
            key_type: params.key_type,
            key_size: params.key_size,
            comment: params.comment,
            public_key,
            private_key,
            fingerprint,
            created_at: Utc::now(),
            last_used: None,
        })
    }
    
    // 生成模拟RSA密钥
    fn generate_mock_rsa_key(_bits: u32) -> AppResult<(String, String)> {
        let private_key = format!(
            "-----BEGIN RSA PRIVATE KEY-----\n{}\n-----END RSA PRIVATE KEY-----",
            Self::generate_random_base64(256)
        );
        
        let public_key = format!(
            "ssh-rsa {} user@host",
            Self::generate_random_base64(372)
        );
        
        Ok((private_key, public_key))
    }
    
    // 生成模拟Ed25519密钥
    fn generate_mock_ed25519_key() -> AppResult<(String, String)> {
        let private_key = format!(
            "-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----",
            Self::generate_random_base64(44)
        );
        
        let public_key = format!(
            "ssh-ed25519 {} user@host",
            Self::generate_random_base64(44)
        );
        
        Ok((private_key, public_key))
    }
    
    // 生成模拟ECDSA密钥
    fn generate_mock_ecdsa_key(bits: u32) -> AppResult<(String, String)> {
        let private_key = format!(
            "-----BEGIN EC PRIVATE KEY-----\n{}\n-----END EC PRIVATE KEY-----",
            Self::generate_random_base64(128)
        );
        
        let curve = match bits {
            256 => "nistp256",
            384 => "nistp384",
            521 => "nistp521",
            _ => return Err(AppError::KeyGenerationError("不支持的ECDSA密钥长度".to_string())),
        };
        
        let public_key = format!(
            "ecdsa-sha2-{} {} user@host",
            curve,
            Self::generate_random_base64(128)
        );
        
        Ok((private_key, public_key))
    }
    
    // 生成随机base64字符串
    fn generate_random_base64(length: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
        base64::encode(&bytes)
    }
    
    // 计算密钥指纹
    fn calculate_fingerprint(public_key: &str) -> AppResult<String> {
        // 提取base64部分
        let parts: Vec<&str> = public_key.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(AppError::KeyGenerationError("无效的公钥格式".to_string()));
        }
        
        let key_data = base64::decode(parts[1])
            .map_err(|_| AppError::KeyGenerationError("公钥解码失败".to_string()))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&key_data);
        let hash = hasher.finalize();
        
        Ok(format!("SHA256:{}", base64::encode(&hash)))
    }
}