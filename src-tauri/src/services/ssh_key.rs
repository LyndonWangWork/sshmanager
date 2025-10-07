use crate::error::{AppError, AppResult};
use crate::types::{KeyGenerationParams, SshKeyPair, SshKeyType};
use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;

// 导入ssh-key库
use base64::{engine::general_purpose, Engine as _};
use ssh_key::private::{EcdsaKeypair, Ed25519Keypair, RsaKeypair};
use ssh_key::EcdsaCurve;
use ssh_key::{LineEnding, PrivateKey, PublicKey};
use zeroize::Zeroizing;

// 为 ssh_key 的 RNG 接口提供基于 OS 的加密安全 RNG 适配器
use ssh_key::rand_core::{CryptoRng as SshCryptoRng, RngCore as SshRngCore};

struct OsCryptoRng;

impl SshRngCore for OsCryptoRng {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        getrandom::getrandom(&mut buf).expect("OS RNG failure");
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).expect("OS RNG failure");
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom::getrandom(dest).expect("OS RNG failure");
    }
}

impl SshCryptoRng for OsCryptoRng {}

pub struct SshKeyService;

impl SshKeyService {
    /// 生成SSH密钥对（使用ssh-key库）
    pub fn generate_key_pair(params: KeyGenerationParams) -> AppResult<SshKeyPair> {
        let mut rng = OsCryptoRng;

        // 使用ssh-key库生成密钥对
        let (private_key, public_key) = match params.key_type {
            SshKeyType::Ed25519 => {
                let ed25519_keypair = Ed25519Keypair::random(&mut rng);
                let private_key = PrivateKey::from(ed25519_keypair);
                let public_key = PublicKey::from(&private_key);
                (private_key, public_key)
            }
            SshKeyType::Rsa => {
                let rsa_keypair = RsaKeypair::random(&mut rng, params.key_size as usize)
                    .map_err(|e| AppError::KeyGenerationError(format!("RSA密钥生成失败: {}", e)))?;
                let private_key = PrivateKey::from(rsa_keypair);
                let public_key = PublicKey::from(&private_key);
                (private_key, public_key)
            }
            SshKeyType::Ecdsa => {
                let ecdsa_keypair = match params.key_size {
                    256 => EcdsaKeypair::random(&mut rng, EcdsaCurve::NistP256).map_err(|e| {
                        AppError::KeyGenerationError(format!("ECDSA P-256密钥生成失败: {}", e))
                    })?,
                    384 => EcdsaKeypair::random(&mut rng, EcdsaCurve::NistP384).map_err(|e| {
                        AppError::KeyGenerationError(format!("ECDSA P-384密钥生成失败: {}", e))
                    })?,
                    521 => EcdsaKeypair::random(&mut rng, EcdsaCurve::NistP521).map_err(|e| {
                        AppError::KeyGenerationError(format!("ECDSA P-521密钥生成失败: {}", e))
                    })?,
                    _ => {
                        return Err(AppError::KeyGenerationError(
                            "不支持的ECDSA密钥长度".to_string(),
                        ))
                    }
                };
                let private_key = PrivateKey::from(ecdsa_keypair);
                let public_key = PublicKey::from(&private_key);
                (private_key, public_key)
            }
        };

        // 生成私钥（OpenSSH格式）
        let private_key_pem: Zeroizing<String> = private_key
            .to_openssh(LineEnding::LF)
            .map_err(|e| AppError::KeyGenerationError(format!("私钥格式转换失败: {}", e)))?;

        // 生成公钥
        let public_key_openssh = public_key
            .to_openssh()
            .map_err(|e| AppError::KeyGenerationError(format!("公钥格式转换失败: {}", e)))?;

        let fingerprint = Self::calculate_fingerprint(&public_key_openssh)?;

        Ok(SshKeyPair {
            id: Uuid::new_v4().to_string(),
            name: params.name,
            key_type: params.key_type,
            key_size: params.key_size,
            comment: params.comment,
            public_key: public_key_openssh,
            private_key: private_key_pem.to_string(),
            fingerprint,
            created_at: Utc::now(),
            last_used: None,
        })
    }

    /// 计算密钥指纹（SHA256）
    fn calculate_fingerprint(public_key: &str) -> AppResult<String> {
        // 提取base64部分
        let parts: Vec<&str> = public_key.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(AppError::KeyGenerationError("无效的公钥格式".to_string()));
        }

        let key_data = general_purpose::STANDARD
            .decode(parts[1])
            .map_err(|_| AppError::KeyGenerationError("公钥解码失败".to_string()))?;

        let mut hasher = Sha256::new();
        hasher.update(&key_data);
        let hash = hasher.finalize();

        Ok(format!(
            "SHA256:{}",
            general_purpose::STANDARD.encode(&hash)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_key_generation() {
        let params = KeyGenerationParams {
            name: "test-key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "test comment".to_string(),
            passphrase: None,
        };

        let result = SshKeyService::generate_key_pair(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.key_type, SshKeyType::Ed25519);
        assert_eq!(key_pair.key_size, 256);
        assert!(key_pair
            .private_key
            .contains("-----BEGIN OPENSSH PRIVATE KEY-----"));
        assert!(key_pair
            .private_key
            .contains("-----END OPENSSH PRIVATE KEY-----"));
        assert!(key_pair.public_key.starts_with("ssh-ed25519 "));

        // 验证私钥格式
        println!("Generated private key:\n{}", key_pair.private_key);
        println!("Generated public key: {}", key_pair.public_key);
    }
}
