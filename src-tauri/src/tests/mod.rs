// 测试模块
use crate::services::{CryptoService, SshKeyService};
use crate::types::{SshKeyType, KeyGenerationParams};

// 加密服务测试
#[path = "../../tests/crypto_test.rs"]
mod crypto_test;

// SSH 密钥服务测试  
#[path = "../../tests/ssh_key_test.rs"]
mod ssh_key_test;

// 集成测试
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_key_generation_flow() {
        let crypto_service = CryptoService::new().unwrap();
        let ssh_service = SshKeyService::new().unwrap();

        // 生成一个密钥
        let params = KeyGenerationParams {
            name: "Integration Test Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "integration@test.com".to_string(),
            passphrase: "".to_string(),
        };

        let key_pair = ssh_service.generate_key(params).unwrap();

        // 测试加密私钥
        let master_password = "test_master_password";
        let encrypted_private_key = crypto_service.encrypt(&key_pair.private_key, master_password);
        assert!(encrypted_private_key.is_ok());

        // 测试解密私钥
        let decrypted_private_key = crypto_service.decrypt(&encrypted_private_key.unwrap(), master_password);
        assert!(decrypted_private_key.is_ok());
        assert_eq!(decrypted_private_key.unwrap(), key_pair.private_key);
    }

    #[test]
    fn test_password_hashing_consistency() {
        let crypto_service = CryptoService::new().unwrap();
        let password = "user_master_password";
        let salt = "user_salt";

        let hash1 = crypto_service.hash_password(password, salt).unwrap();
        let hash2 = crypto_service.hash_password(password, salt).unwrap();

        assert_eq!(hash1, hash2);
        assert!(hash1.len() > 0);
    }

    #[test]
    fn test_multiple_key_types() {
        let ssh_service = SshKeyService::new().unwrap();
        
        let key_types = vec![
            (SshKeyType::RSA, 2048),
            (SshKeyType::Ed25519, 256),
            (SshKeyType::ECDSA, 256),
        ];

        for (key_type, key_size) in key_types {
            let params = KeyGenerationParams {
                name: format!("Test {} Key", key_type.to_string()),
                key_type,
                key_size,
                comment: format!("{}@test.com", key_type.to_string().to_lowercase()),
                passphrase: "".to_string(),
            };

            let result = ssh_service.generate_key(params);
            assert!(result.is_ok(), "Failed to generate {} key", key_type.to_string());
            
            let key_pair = result.unwrap();
            assert_eq!(key_pair.key_type, key_type);
            assert_eq!(key_pair.key_size, key_size);
        }
    }
}