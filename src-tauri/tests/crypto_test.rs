#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_service_new() {
        let crypto_service = CryptoService::new();
        assert!(crypto_service.is_ok());
    }

    #[test]
    fn test_hash_password() {
        let crypto_service = CryptoService::new().unwrap();
        let password = "test_password";
        let salt = "test_salt";
        
        let hash1 = crypto_service.hash_password(password, salt);
        let hash2 = crypto_service.hash_password(password, salt);
        
        assert!(hash1.is_ok());
        assert!(hash2.is_ok());
        assert_eq!(hash1.unwrap(), hash2.unwrap());
    }

    #[test]
    fn test_hash_password_different_salts() {
        let crypto_service = CryptoService::new().unwrap();
        let password = "test_password";
        let salt1 = "salt1";
        let salt2 = "salt2";
        
        let hash1 = crypto_service.hash_password(password, salt1).unwrap();
        let hash2 = crypto_service.hash_password(password, salt2).unwrap();
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let crypto_service = CryptoService::new().unwrap();
        let data = "test data to encrypt";
        let key = "encryption_key";
        
        let encrypted = crypto_service.encrypt(data, key);
        assert!(encrypted.is_ok());
        
        let decrypted = crypto_service.decrypt(&encrypted.unwrap(), key);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), data);
    }

    #[test]
    fn test_encrypt_decrypt_with_different_keys() {
        let crypto_service = CryptoService::new().unwrap();
        let data = "test data to encrypt";
        let key1 = "encryption_key1";
        let key2 = "encryption_key2";
        
        let encrypted = crypto_service.encrypt(data, key1).unwrap();
        let decrypted = crypto_service.decrypt(&encrypted, key2);
        
        // Should fail with different key
        assert!(decrypted.is_err() || decrypted.unwrap() != data);
    }

    #[test]
    fn test_generate_random() {
        let crypto_service = CryptoService::new().unwrap();
        
        let random1 = crypto_service.generate_random(32);
        let random2 = crypto_service.generate_random(32);
        
        assert!(random1.is_ok());
        assert!(random2.is_ok());
        
        let random1 = random1.unwrap();
        let random2 = random2.unwrap();
        
        assert_eq!(random1.len(), 32);
        assert_eq!(random2.len(), 32);
        assert_ne!(random1, random2);
    }

    #[test]
    fn test_generate_random_different_lengths() {
        let crypto_service = CryptoService::new().unwrap();
        
        let random16 = crypto_service.generate_random(16).unwrap();
        let random32 = crypto_service.generate_random(32).unwrap();
        let random64 = crypto_service.generate_random(64).unwrap();
        
        assert_eq!(random16.len(), 16);
        assert_eq!(random32.len(), 32);
        assert_eq!(random64.len(), 64);
    }

    #[test]
    fn test_encrypt_empty_string() {
        let crypto_service = CryptoService::new().unwrap();
        let data = "";
        let key = "encryption_key";
        
        let encrypted = crypto_service.encrypt(data, key);
        assert!(encrypted.is_ok());
        
        let decrypted = crypto_service.decrypt(&encrypted.unwrap(), key);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), data);
    }

    #[test]
    fn test_encrypt_large_data() {
        let crypto_service = CryptoService::new().unwrap();
        let data = "a".repeat(1000); // 1KB of data
        let key = "encryption_key";
        
        let encrypted = crypto_service.encrypt(&data, key);
        assert!(encrypted.is_ok());
        
        let decrypted = crypto_service.decrypt(&encrypted.unwrap(), key);
        assert!(decrypted.is_ok());
        assert_eq!(decrypted.unwrap(), data);
    }

    #[test]
    fn test_hash_empty_password() {
        let crypto_service = CryptoService::new().unwrap();
        let password = "";
        let salt = "test_salt";
        
        let hash = crypto_service.hash_password(password, salt);
        assert!(hash.is_ok());
        assert!(!hash.unwrap().is_empty());
    }

    #[test]
    fn test_hash_empty_salt() {
        let crypto_service = CryptoService::new().unwrap();
        let password = "test_password";
        let salt = "";
        
        let hash = crypto_service.hash_password(password, salt);
        assert!(hash.is_ok());
        assert!(!hash.unwrap().is_empty());
    }

    #[test] 
    fn test_deterministic_hashing() {
        let crypto_service = CryptoService::new().unwrap();
        let password = "test_password";
        let salt = "test_salt";
        
        // Hash the same input multiple times
        let hashes: Vec<String> = (0..5)
            .map(|_| crypto_service.hash_password(password, salt).unwrap())
            .collect();
        
        // All hashes should be identical
        for hash in &hashes[1..] {
            assert_eq!(&hashes[0], hash);
        }
    }
}