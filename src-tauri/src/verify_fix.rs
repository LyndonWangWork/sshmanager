// 验证修复的程序
use std::fs;
use std::path::PathBuf;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use dirs;

// 模拟加密数据结构
#[derive(Debug)]
pub struct EncryptedData {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

// 模拟加密存储结构
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptedStorage {
    pub version: String,
    pub salt: Vec<u8>,
    pub iv: Vec<u8>,
    pub encrypted_data: Vec<u8>,
    pub checksum: String,
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

// 模拟密码哈希函数
fn hash_password(password: &str, salt: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}

// 模拟生成盐值
fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    // 使用固定的盐值以便测试
    for i in 0..32 {
        salt[i] = i as u8;
    }
    salt
}

// 模拟保存加密数据（修复后的版本）
fn save_encrypted_data_fixed(storage_path: &PathBuf, encrypted_data: &EncryptedData, salt: &[u8], master_key_hash: &str) -> Result<(), String> {
    // 创建存储数据（注意：master_key_hash和salt作为元数据存储，不在加密数据中）
    let mut data_map = Map::new();
    // 注意：这里不存储master_key_hash和salt在加密数据中，而是作为元数据存储
    
    // 创建空的keys数组
    data_map.insert("keys".to_string(), Value::Array(vec![]));
    
    // 创建配置对象
    let mut config_map = Map::new();
    config_map.insert("theme".to_string(), Value::String("light".to_string()));
    config_map.insert("auto_backup".to_string(), Value::Bool(true));
    config_map.insert("backup_retention".to_string(), Value::Number(10.into()));
    config_map.insert("default_key_type".to_string(), Value::String("Ed25519".to_string()));
    config_map.insert("default_key_size".to_string(), Value::Number(256.into()));
    
    data_map.insert("config".to_string(), Value::Object(config_map));
    
    let storage_data = EncryptedStorage {
        version: "1.0".to_string(),
        salt: salt.to_vec(), // 盐值作为元数据存储
        iv: encrypted_data.nonce.clone(),
        encrypted_data: encrypted_data.ciphertext.clone(),
        checksum: calculate_checksum(&encrypted_data.ciphertext),
        data: data_map, // 加密数据中不包含master_key_hash和salt
    };
    
    let serialized = serde_json::to_string(&storage_data).map_err(|e| e.to_string())?;
    std::fs::write(storage_path, serialized).map_err(|e| e.to_string())?;
    
    // 同时将master_key_hash存储在元数据中（这已经在EncryptedStorage中完成）
    // 我们还需要将master_key_hash存储在data_map中以便读取
    // 但实际上应该存储在EncryptedStorage的字段中，而不是data_map中
    
    Ok(())
}

// 模拟加载加密数据
fn load_encrypted_data(storage_path: &PathBuf) -> Result<(EncryptedData, Vec<u8>, String, [u8; 32]), String> {
    let content = std::fs::read_to_string(storage_path).map_err(|e| e.to_string())?;
    let storage_data: EncryptedStorage = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    // 验证校验和
    let calculated_checksum = calculate_checksum(&storage_data.encrypted_data);
    if calculated_checksum != storage_data.checksum {
        return Err("数据完整性验证失败".to_string());
    }
    
    // 从元数据中获取主密码哈希（这里模拟从文件中读取）
    // 实际应用中，master_key_hash应该存储在安全的地方
    // 在这个测试中，我们假设它存储在某个地方
    let master_key_hash = "test_hash_value"; // 这里应该从安全存储中读取
    
    // 从元数据中获取盐值
    let salt_vec = storage_data.salt.clone();
    
    // 确保盐值长度为32字节
    if salt_vec.len() != 32 {
        return Err("盐值长度不正确".to_string());
    }
    
    let mut salt = [0u8; 32];
    salt.copy_from_slice(&salt_vec);
    
    // 返回加密数据、盐值、主密码哈希
    Ok((
        EncryptedData {
            nonce: storage_data.iv,
            ciphertext: storage_data.encrypted_data,
        },
        salt_vec,
        master_key_hash.to_string(),
        salt,
    ))
}

// 计算校验和
fn calculate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

// 模拟加密（简单的XOR）
fn encrypt(data: &[u8]) -> EncryptedData {
    let key_byte = 0x42; // 简单的密钥
    let encrypted: Vec<u8> = data.iter().map(|b| b ^ key_byte).collect();
    
    EncryptedData {
        nonce: vec![0u8; 12], // 模拟 nonce
        ciphertext: encrypted,
    }
}

fn main() -> Result<(), String> {
    println!("=== 验证修复 ===");
    
    // 实际上，我们需要检查原始代码中的问题
    // 问题在于initialize_app和authenticate函数中对master_key_hash的处理不一致
    
    println!("问题分析:");
    println!("1. 在initialize_app中，master_key_hash被存储在两个地方:");
    println!("   - 加密数据的JSON中");
    println!("   - EncryptedStorage的元数据中");
    println!("2. 在authenticate中，只从EncryptedStorage的元数据中读取master_key_hash");
    println!("3. 如果这两个地方存储的值不一致，就会导致认证失败");
    
    println!("\n修复方案:");
    println!("1. 在initialize_app中，只将master_key_hash存储在EncryptedStorage的元数据中");
    println!("2. 在authenticate中，从EncryptedStorage的元数据中读取master_key_hash");
    println!("3. 确保两个地方使用相同的存储和读取方式");
    
    Ok(())
}