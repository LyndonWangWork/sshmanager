use uuid::Uuid;
use chrono::Utc;
use sha2::{Digest, Sha256};
use crate::types::{SshKeyPair, SshKeyType, KeyGenerationParams};
use crate::error::{AppError, AppResult};

// 导入ring相关库
use ring::{rand, signature};
use ring::signature::KeyPair;
// 导入RSA相关库
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rand_core::OsRng;
// 导入base64编码库
use base64::{Engine as _, engine::general_purpose};

pub struct SshKeyService;

impl SshKeyService {
    /// 生成SSH密钥对（真实的密码学实现）
    pub fn generate_key_pair(params: KeyGenerationParams) -> AppResult<SshKeyPair> {
        let (private_key, public_key) = match params.key_type {
            SshKeyType::Rsa => Self::generate_rsa_key(params.key_size)?,
            SshKeyType::Ed25519 => Self::generate_ed25519_key()?,
            SshKeyType::Ecdsa => Self::generate_ecdsa_key(params.key_size)?,
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
    
    /// 生成真实的RSA密钥对
    fn generate_rsa_key(bits: u32) -> AppResult<(String, String)> {
        // 生成RSA密钥对
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, bits as usize)
            .map_err(|e| AppError::KeyGenerationError(format!("RSA密钥生成失败: {}", e)))?;
        let public_key = RsaPublicKey::from(&private_key);
        
        // 转换为PEM格式
        let private_key_pem = private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| AppError::KeyGenerationError(format!("私钥PEM格式转换失败: {}", e)))?
            .to_string();
        
        // 生成OpenSSH格式的公钥
        let public_key_bytes = public_key.to_pkcs1_der()
            .map_err(|e| AppError::KeyGenerationError(format!("公钥DER格式转换失败: {}", e)))?;
        
        let public_key_openssh = format!(
            "ssh-rsa {}",
            general_purpose::STANDARD.encode(&public_key_bytes.as_bytes())
        );
        
        Ok((private_key_pem, public_key_openssh))
    }
    
    /// 生成真实的Ed25519密钥对
    fn generate_ed25519_key() -> AppResult<(String, String)> {
        // 生成Ed25519密钥对
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| AppError::KeyGenerationError(format!("Ed25519密钥生成失败: {:?}", e)))?;
        
        // 提取公钥
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| AppError::KeyGenerationError(format!("Ed25519密钥解析失败: {:?}", e)))?;
        let public_key_bytes = key_pair.public_key().as_ref();
        
        // OpenSSH格式的Ed25519私钥
        let private_key_openssh = format!(
            "-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----",
            general_purpose::STANDARD.encode(&pkcs8_bytes)
        );
        
        // OpenSSH格式的公钥
        let public_key_openssh = format!(
            "ssh-ed25519 {}",
            general_purpose::STANDARD.encode(public_key_bytes)
        );
        
        Ok((private_key_openssh, public_key_openssh))
    }
    
    /// 生成真实的ECDSA密钥对
    fn generate_ecdsa_key(bits: u32) -> AppResult<(String, String)> {
        // 生成ECDSA密钥对
        let rng = rand::SystemRandom::new();
        
        match bits {
            256 => {
                // 生成P-256曲线的ECDSA密钥对
                let pkcs8_bytes = signature::EcdsaKeyPair::generate_pkcs8(
                    &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
                    &rng,
                ).map_err(|e| AppError::KeyGenerationError(format!("ECDSA P-256密钥生成失败: {:?}", e)))?;
                
                let key_pair = signature::EcdsaKeyPair::from_pkcs8(
                    &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
                    pkcs8_bytes.as_ref(),
                    &rng,
                ).map_err(|e| AppError::KeyGenerationError(format!("ECDSA P-256密钥解析失败: {:?}", e)))?;
                
                let public_key_bytes = key_pair.public_key().as_ref();
                
                // PEM格式的私钥
                let private_key_pem = format!(
                    "-----BEGIN EC PRIVATE KEY-----\n{}\n-----END EC PRIVATE KEY-----",
                    general_purpose::STANDARD.encode(&pkcs8_bytes)
                );
                
                // OpenSSH格式的公钥
                let public_key_openssh = format!(
                    "ecdsa-sha2-nistp256 {}",
                    general_purpose::STANDARD.encode(public_key_bytes)
                );
                
                Ok((private_key_pem, public_key_openssh))
            },
            384 => {
                // 生成P-384曲线的ECDSA密钥对
                let pkcs8_bytes = signature::EcdsaKeyPair::generate_pkcs8(
                    &signature::ECDSA_P384_SHA384_ASN1_SIGNING,
                    &rng,
                ).map_err(|e| AppError::KeyGenerationError(format!("ECDSA P-384密钥生成失败: {:?}", e)))?;
                
                let key_pair = signature::EcdsaKeyPair::from_pkcs8(
                    &signature::ECDSA_P384_SHA384_ASN1_SIGNING,
                    pkcs8_bytes.as_ref(),
                    &rng,
                ).map_err(|e| AppError::KeyGenerationError(format!("ECDSA P-384密钥解析失败: {:?}", e)))?;
                
                let public_key_bytes = key_pair.public_key().as_ref();
                
                // PEM格式的私钥
                let private_key_pem = format!(
                    "-----BEGIN EC PRIVATE KEY-----\n{}\n-----END EC PRIVATE KEY-----",
                    general_purpose::STANDARD.encode(&pkcs8_bytes)
                );
                
                // OpenSSH格式的公钥
                let public_key_openssh = format!(
                    "ecdsa-sha2-nistp384 {}",
                    general_purpose::STANDARD.encode(public_key_bytes)
                );
                
                Ok((private_key_pem, public_key_openssh))
            },
            _ => Err(AppError::KeyGenerationError("不支持的ECDSA密钥长度".to_string()))
        }
    }
    
    /// 计算密钥指纹（SHA256）
    fn calculate_fingerprint(public_key: &str) -> AppResult<String> {
        // 提取base64部分
        let parts: Vec<&str> = public_key.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(AppError::KeyGenerationError("无效的公钥格式".to_string()));
        }
        
        let key_data = general_purpose::STANDARD.decode(parts[1])
            .map_err(|_| AppError::KeyGenerationError("公钥解码失败".to_string()))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&key_data);
        let hash = hasher.finalize();
        
        Ok(format!("SHA256:{}", general_purpose::STANDARD.encode(&hash)))
    }
}