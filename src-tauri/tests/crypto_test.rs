#[cfg(test)]
mod tests {
    use ssh_key_manager_lib::services::CryptoService;

    #[test]
    fn test_crypto_service_new() {
        let _crypto_service = CryptoService::new();
        assert!(true); // CryptoService::new() always succeeds
    }

    #[test]
    fn test_derive_key() {
        let password = "test_password";
        let salt = [0u8; 32];

        let key1 = CryptoService::derive_key(password, &salt);
        let key2 = CryptoService::derive_key(password, &salt);

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_different_salts() {
        let password = "test_password";
        let salt1 = [1u8; 32];
        let salt2 = [2u8; 32];

        let key1 = CryptoService::derive_key(password, &salt1);
        let key2 = CryptoService::derive_key(password, &salt2);

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_set_master_key() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";

        let result = crypto_service.set_master_key(password);
        assert!(result.is_ok());
        assert!(crypto_service.is_authenticated());
        assert!(crypto_service.get_salt().is_some());
        assert!(crypto_service.get_master_key_hash().is_some());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";
        crypto_service.set_master_key(password).unwrap();

        let data = "test data to encrypt";

        let encrypted = crypto_service.encrypt(data.as_bytes()).unwrap();
        let decrypted = crypto_service.decrypt(&encrypted).unwrap();
        assert_eq!(String::from_utf8(decrypted).unwrap(), data);
    }

    #[test]
    fn test_encrypt_decrypt_with_different_keys() {
        let mut crypto_service1 = CryptoService::new();
        let mut crypto_service2 = CryptoService::new();

        // Set up service 1
        crypto_service1.set_master_key("key1").unwrap();

        // Set up service 2
        crypto_service2.set_master_key("key2").unwrap();

        let data = "test data to encrypt";

        let encrypted1 = crypto_service1.encrypt(data.as_bytes()).unwrap();
        let encrypted2 = crypto_service2.encrypt(data.as_bytes()).unwrap();

        // With the new AES-GCM implementation, the encrypted data should be different
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    }

    #[test]
    fn test_generate_random() {
        let salt1 = CryptoService::generate_salt();
        let salt2 = CryptoService::generate_salt();

        assert_eq!(salt1.len(), 32);
        assert_eq!(salt2.len(), 32);
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_encrypt_empty_string() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";
        crypto_service.set_master_key(password).unwrap();

        let data = "";

        let encrypted = crypto_service.encrypt(data.as_bytes()).unwrap();
        let decrypted = crypto_service.decrypt(&encrypted).unwrap();
        assert_eq!(String::from_utf8(decrypted).unwrap(), data);
    }

    #[test]
    fn test_encrypt_large_data() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";
        crypto_service.set_master_key(password).unwrap();

        let data = "a".repeat(1000); // 1KB of data

        let encrypted = crypto_service.encrypt(data.as_bytes()).unwrap();
        let decrypted = crypto_service.decrypt(&encrypted).unwrap();
        assert_eq!(String::from_utf8(decrypted).unwrap(), data);
    }

    #[test]
    fn test_verify_password() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";
        crypto_service.set_master_key(password).unwrap();

        assert!(crypto_service.verify_password(password));
        assert!(!crypto_service.verify_password("wrong_password"));
    }

    #[test]
    fn test_encrypt_twice_produces_distinct_nonce_and_ciphertext() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";
        crypto_service.set_master_key(password).unwrap();

        let data = b"repeatable data";

        let e1 = crypto_service.encrypt(data).unwrap();
        let e2 = crypto_service.encrypt(data).unwrap();

        assert_ne!(e1.nonce, e2.nonce, "nonces should differ");
        assert_ne!(e1.ciphertext, e2.ciphertext, "ciphertexts should differ");
    }

    #[test]
    fn test_clear_master_key() {
        let mut crypto_service = CryptoService::new();
        let password = "test_password";
        crypto_service.set_master_key(password).unwrap();

        assert!(crypto_service.is_authenticated());
        crypto_service.clear_master_key();
        assert!(!crypto_service.is_authenticated());
    }

    #[test]
    fn test_hash_empty_password() {
        let password = "";
        let salt = [0u8; 32];

        let hash = CryptoService::hash_password(password, &salt);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_empty_salt() {
        let password = "test_password";
        let salt = [0u8; 32]; // Empty salt is still 32 bytes of zeros

        let hash = CryptoService::hash_password(password, &salt);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_deterministic_hashing() {
        let password = "test_password";
        let salt = [0u8; 32];

        // Hash the same input multiple times
        let hashes: Vec<String> = (0..5)
            .map(|_| CryptoService::hash_password(password, &salt))
            .collect();

        // All hashes should be identical
        for hash in &hashes[1..] {
            assert_eq!(&hashes[0], hash);
        }
    }
}
