#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SshKeyPair;

    #[test]
    fn test_ssh_key_service_new() {
        let ssh_service = SshKeyService::new();
        assert!(ssh_service.is_ok());
    }

    #[test]
    fn test_generate_rsa_key() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Test RSA Key".to_string(),
            key_type: SshKeyType::RSA,
            key_size: 2048,
            comment: "test@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.name, "Test RSA Key");
        assert_eq!(key_pair.key_type, SshKeyType::RSA);
        assert_eq!(key_pair.key_size, 2048);
        assert_eq!(key_pair.comment, "test@example.com");
        assert!(!key_pair.public_key.is_empty());
        assert!(!key_pair.private_key.is_empty());
        assert!(!key_pair.fingerprint.is_empty());
        assert!(!key_pair.id.is_empty());
    }

    #[test]
    fn test_generate_ed25519_key() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Test Ed25519 Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "test@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.name, "Test Ed25519 Key");
        assert_eq!(key_pair.key_type, SshKeyType::Ed25519);
        assert_eq!(key_pair.key_size, 256);
        assert!(key_pair.public_key.starts_with("ssh-ed25519"));
        assert!(key_pair.private_key.contains("BEGIN OPENSSH PRIVATE KEY"));
    }

    #[test]
    fn test_generate_ecdsa_key() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Test ECDSA Key".to_string(),
            key_type: SshKeyType::ECDSA,
            key_size: 256,
            comment: "test@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.name, "Test ECDSA Key");
        assert_eq!(key_pair.key_type, SshKeyType::ECDSA);
        assert!(key_pair.public_key.starts_with("ecdsa-sha2-nistp256"));
    }

    #[test]
    fn test_generate_key_with_passphrase() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Test Protected Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "protected@example.com".to_string(),
            passphrase: "secure_passphrase".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.name, "Test Protected Key");
        assert_eq!(key_pair.comment, "protected@example.com");
        // Note: In a real implementation, the private key would be encrypted
        assert!(!key_pair.private_key.is_empty());
    }

    #[test]
    fn test_generate_multiple_keys_have_different_ids() {
        let ssh_service = SshKeyService::new().unwrap();
        let params1 = KeyGenerationParams {
            name: "Key 1".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "key1@example.com".to_string(),
            passphrase: "".to_string(),
        };
        let params2 = KeyGenerationParams {
            name: "Key 2".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "key2@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let key1 = ssh_service.generate_key(params1).unwrap();
        let key2 = ssh_service.generate_key(params2).unwrap();

        assert_ne!(key1.id, key2.id);
        assert_ne!(key1.public_key, key2.public_key);
        assert_ne!(key1.private_key, key2.private_key);
        assert_ne!(key1.fingerprint, key2.fingerprint);
    }

    #[test]
    fn test_fingerprint_format() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Test Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "test@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let key_pair = ssh_service.generate_key(params).unwrap();
        
        // Fingerprint should start with SHA256:
        assert!(key_pair.fingerprint.starts_with("SHA256:"));
        // Should be followed by base64 characters
        let fingerprint_part = &key_pair.fingerprint[7..]; // Remove "SHA256:" prefix
        assert!(fingerprint_part.len() > 0);
        assert!(fingerprint_part.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '='));
    }

    #[test]
    fn test_key_creation_timestamp() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Timestamp Test Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "timestamp@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let before = chrono::Utc::now();
        let key_pair = ssh_service.generate_key(params).unwrap();
        let after = chrono::Utc::now();

        let created_at = chrono::DateTime::parse_from_rfc3339(&key_pair.created_at).unwrap();
        assert!(created_at >= before.into());
        assert!(created_at <= after.into());
    }

    #[test]
    fn test_empty_comment_handling() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "No Comment Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "".to_string(),
            passphrase: "".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.comment, "");
        assert!(key_pair.public_key.ends_with(" ")); // Should still have trailing space for empty comment
    }

    #[test]
    fn test_long_name_handling() {
        let ssh_service = SshKeyService::new().unwrap();
        let long_name = "a".repeat(255); // Very long name
        let params = KeyGenerationParams {
            name: long_name.clone(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "longname@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.name, long_name);
    }

    #[test]
    fn test_special_characters_in_comment() {
        let ssh_service = SshKeyService::new().unwrap();
        let params = KeyGenerationParams {
            name: "Special Chars Key".to_string(),
            key_type: SshKeyType::Ed25519,
            key_size: 256,
            comment: "user+tag@example.com".to_string(),
            passphrase: "".to_string(),
        };

        let result = ssh_service.generate_key(params);
        assert!(result.is_ok());

        let key_pair = result.unwrap();
        assert_eq!(key_pair.comment, "user+tag@example.com");
        assert!(key_pair.public_key.contains("user+tag@example.com"));
    }
}