use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SshKeyType {
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "Ed25519")]
    Ed25519,
    #[serde(rename = "ECDSA")]
    Ecdsa,
}

impl std::fmt::Display for SshKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SshKeyType::Rsa => write!(f, "RSA"),
            SshKeyType::Ed25519 => write!(f, "Ed25519"),
            SshKeyType::Ecdsa => write!(f, "ECDSA"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenerationParams {
    pub name: String,
    pub key_type: SshKeyType,
    pub key_size: u32,
    pub comment: String,
    pub passphrase: Option<String>, // 密钥密码（可选）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyPair {
    pub id: String,
    pub name: String,
    pub key_type: SshKeyType,
    pub key_size: u32,
    pub comment: String,
    pub public_key: String,
    pub private_key: String,
    pub fingerprint: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedStorage {
    pub version: String,
    pub salt: Vec<u8>,
    pub master_key_hash: String,
    pub iv: Vec<u8>,
    pub encrypted_data: Vec<u8>,
    pub checksum: String,
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
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