// 测试模块
// 移除对独立测试文件的引用，避免重复编译

#[cfg(test)]
mod integration_tests {
    use crate::services::{CryptoService, SshKeyService};
    use crate::types::{SshKeyType, KeyGenerationParams};

    #[test]
    fn test_full_key_generation_flow() {
        // 生成一个密钥
        let params = KeyGenerationParams {
            name: "Integration Test Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "integration@test.com".to_string(),
            passphrase: None,
        };

        let key_pair = SshKeyService::generate_key_pair(params).unwrap();

        // 测试加密私钥
        let mut crypto_service = CryptoService::new();
        let salt = [0u8; 32];
        let master_key_hash = CryptoService::hash_password("test_master_password", &salt);
        crypto_service.set_master_key_hash(master_key_hash);
        crypto_service.set_salt(salt);
        
        let encrypted_private_key = crypto_service.encrypt(key_pair.private_key.as_bytes()).unwrap();
        assert!(encrypted_private_key.ciphertext.len() > 0);

        // 测试解密私钥
        let decrypted_private_key = crypto_service.decrypt(&encrypted_private_key).unwrap();
        assert_eq!(String::from_utf8(decrypted_private_key).unwrap(), key_pair.private_key);
    }

    #[test]
    fn test_password_hashing_consistency() {
        let password = "user_master_password";
        let salt = [1u8; 32];

        let hash1 = CryptoService::hash_password(password, &salt);
        let hash2 = CryptoService::hash_password(password, &salt);

        assert_eq!(hash1, hash2);
        assert!(hash1.len() > 0);
    }

    #[test]
    fn test_multiple_key_types() {
        let key_types = vec![
            (SshKeyType::Rsa, 2048),
            (SshKeyType::Ed25519, 256),
            (SshKeyType::Ecdsa, 256),
        ];

        for (key_type, key_size) in key_types {
            let params = KeyGenerationParams {
                name: format!("Test {:?} Key", key_type),
                key_type: key_type.clone(), // Clone to avoid move
                key_size,
                comment: format!("{}@test.com", format!("{:?}", key_type).to_lowercase()),
                passphrase: None,
            };

            let result = SshKeyService::generate_key_pair(params);
            assert!(result.is_ok(), "Failed to generate {:?} key", key_type);
            
            let key_pair = result.unwrap();
            assert_eq!(key_pair.key_type, key_type);
            assert_eq!(key_pair.key_size, key_size);
        }
    }
}