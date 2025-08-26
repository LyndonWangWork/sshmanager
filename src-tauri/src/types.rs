use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyPair {
    pub id: String,
    pub name: String,
    pub key_type: SshKeyType,
    pub key_size: u32,
    pub comment: String,
    pub public_key: String,
    pub private_key: String, // 加密存储
    pub fingerprint: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SshKeyType {
    Rsa,
    Ed25519,
    Ecdsa,
}

#[derive(Debug, Deserialize)]
pub struct KeyGenerationParams {
    pub name: String,
    pub key_type: SshKeyType,
    pub key_size: u32,
    pub comment: String,
    pub passphrase: Option<String>, // 密钥密码（可选）
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub auto_backup: bool,
    pub backup_retention: u32,
    pub default_key_type: SshKeyType,
    pub default_key_size: u32,
    pub ssh_config_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedStorage {
    pub version: String,
    pub salt: Vec<u8>,
    pub iv: Vec<u8>,
    pub encrypted_data: Vec<u8>,
    pub checksum: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SshHostConfig {
    pub host_pattern: String,
    pub hostname: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
    pub other_options: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SshConfig {
    pub hosts: Vec<SshHostConfig>,
    pub global_settings: std::collections::HashMap<String, String>,
}