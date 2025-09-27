// 测试模块
// 移除对独立测试文件的引用，避免重复编译

#[cfg(test)]
mod integration_tests {
    use crate::services::{CryptoService, SshKeyService, SshConfigService};
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
        crypto_service.set_master_key("test_master_password").unwrap();
        
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

    #[test]
    fn test_save_ssh_config_with_backups_and_retention() {
        use std::fs;
        use std::time::Duration;
        use std::thread::sleep;
        use tempfile::tempdir;

        let dir = tempdir().expect("create temp dir");
        let cfg_path = dir.path().join("config");

        // 首次写入（无备份）
        SshConfigService::save_config("Host a\n  HostName a", cfg_path.to_str(), Some(2)).expect("save first");
        assert!(cfg_path.exists());

        // 第二次写入（生成1个备份）
        sleep(Duration::from_millis(1000));
        SshConfigService::save_config("Host b\n  HostName b", cfg_path.to_str(), Some(2)).expect("save second");
        let backups1 = list_backups(&cfg_path);
        assert_eq!(backups1.len(), 1);

        // 第三次写入（生成第2个备份）
        sleep(Duration::from_millis(1000));
        SshConfigService::save_config("Host c\n  HostName c", cfg_path.to_str(), Some(2)).expect("save third");
        let backups2 = list_backups(&cfg_path);
        assert_eq!(backups2.len(), 2);

        // 第四次写入（触发保留上限=2，最旧的被清理）
        sleep(Duration::from_millis(1000));
        SshConfigService::save_config("Host d\n  HostName d", cfg_path.to_str(), Some(2)).expect("save fourth");
        let backups3 = list_backups(&cfg_path);
        assert_eq!(backups3.len(), 2);

        // 校验当前内容为最后一次写入
        let content = fs::read_to_string(&cfg_path).expect("read final");
        assert!(content.contains("Host d"));

        fn list_backups(path: &std::path::Path) -> Vec<std::path::PathBuf> {
            let parent = path.parent().unwrap();
            let name = path.file_name().unwrap().to_str().unwrap();
            let mut v: Vec<_> = fs::read_dir(parent)
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.file_name().and_then(|s| s.to_str()).map(|n| n.starts_with(&format!("{}{}", name, ".bak."))).unwrap_or(false))
                .collect();
            v.sort();
            v
        }
    }
}