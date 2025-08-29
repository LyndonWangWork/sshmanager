// 端到端测试程序
use std::fs;
use std::path::PathBuf;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

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
    for i in 0..32 {
        salt[i] = (i + 1) as u8; // 使用1-32的值
    }
    salt
}

// 模拟保存加密数据
fn save_encrypted_data(storage_path: &PathBuf, encrypted_data: &EncryptedData, salt: &[u8], master_key_hash: &str) -> Result<(), String> {
    // 创建存储数据
    let mut data_map = Map::new();
    data_map.insert("master_key_hash".to_string(), Value::String(master_key_hash.to_string()));
    data_map.insert("salt".to_string(), Value::Array(salt.iter().map(|&b| Value::Number(b.into())).collect()));
    
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
        salt: salt.to_vec(),
        iv: encrypted_data.nonce.clone(),
        encrypted_data: encrypted_data.ciphertext.clone(),
        checksum: calculate_checksum(&encrypted_data.ciphertext),
        data: data_map,
    };
    
    let serialized = serde_json::to_string(&storage_data).map_err(|e| e.to_string())?;
    std::fs::write(storage_path, serialized).map_err(|e| e.to_string())?;
    
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
    
    // 解析主密码哈希和盐值
    let master_key_hash = storage_data.data.get("master_key_hash")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "缺少主密码哈希".to_string())?;
    
    let salt_array = storage_data.data.get("salt")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "缺少盐值".to_string())?;

    // 将JSON数组转换为[u8]，处理Option类型
    let salt_vec: Result<Vec<u8>, _> = salt_array
        .iter()
        .map(|v| v.as_u64().ok_or("盐值格式错误").map(|u| u as u8))
        .collect();
    
    let salt_vec = salt_vec.map_err(|_| "盐值转换失败")?;

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
        master_key_hash,
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
    println!("=== 端到端认证流程测试 ===");
    
    let password = "test123456";
    println!("测试密码: {}", password);
    
    // 创建临时文件路径
    let storage_path = PathBuf::from("e2e_test_storage.enc");
    println!("存储路径: {:?}", storage_path);
    
    // === 初始化过程 ===
    println!("\n--- 初始化过程 ---");
    
    // 1. 生成随机盐值
    let salt = generate_salt();
    println!("生成盐值: {:?}", &salt[..8]);
    
    // 2. 生成密码哈希
    let master_key_hash = hash_password(password, &salt);
    println!("生成密码哈希: {}...", &master_key_hash[..16]);
    
    // 3. 验证生成的哈希
    let verify_init = hash_password(password, &salt) == master_key_hash;
    println!("初始化时验证: {}", verify_init);
    
    // 4. 创建初始数据
    let initial_data = serde_json::json!({
        "master_key_hash": master_key_hash,
        "salt": salt.to_vec(),
        "keys": [],
        "config": {
            "theme": "light",
            "auto_backup": true,
            "backup_retention": 10,
            "default_key_type": "Ed25519",
            "default_key_size": 256
        }
    });
    println!("初始数据创建完成");
    
    // 5. 加密数据
    let encrypted = encrypt(initial_data.to_string().as_bytes());
    println!("数据加密完成");
    
    // 6. 保存数据
    save_encrypted_data(&storage_path, &encrypted, &salt, &master_key_hash)?;
    println!("数据保存完成");
    
    // === 认证过程 ===
    println!("\n--- 认证过程 ---");
    
    // 1. 加载数据
    let (encrypted_data, salt_vec, stored_hash, loaded_salt) = load_encrypted_data(&storage_path)?;
    println!("数据加载完成");
    println!("存储的哈希值: {}...", &stored_hash[..16]);
    println!("加载的盐值: {:?}", &loaded_salt[..8]);
    println!("原始盐值: {:?}", &salt[..8]);
    println!("盐值是否一致: {}", loaded_salt == salt);
    
    // 2. 验证从data中读取的master_key_hash与从参数传递的是否一致
    let data_hash = initial_data["master_key_hash"].as_str().unwrap();
    println!("data中的哈希值: {}...", &data_hash[..16]);
    println!("参数传递的哈希值: {}...", &master_key_hash[..16]);
    println!("两者是否一致: {}", data_hash == master_key_hash);
    
    // 3. 验证密码
    let input_hash = hash_password(password, &loaded_salt);
    println!("输入密码哈希: {}...", &input_hash[..16]);
    
    let is_valid = input_hash == stored_hash;
    println!("密码验证结果: {}", is_valid);
    
    // 4. 测试错误密码
    let wrong_hash = hash_password("wrongpassword", &loaded_salt);
    let wrong_valid = wrong_hash == stored_hash;
    println!("错误密码验证结果: {}", wrong_valid);
    
    // === 详细调试信息 ===
    println!("\n--- 详细调试信息 ---");
    println!("存储的哈希值完整: {}", stored_hash);
    println!("输入密码哈希完整: {}", input_hash);
    println!("两者是否完全一致: {}", input_hash == stored_hash);
    
    // === 清理 ===
    println!("\n--- 清理 ---");
    if storage_path.exists() {
        fs::remove_file(&storage_path).map_err(|e| e.to_string())?;
        println!("临时文件已删除");
    }
    
    // === 结果 ===
    if is_valid && !wrong_valid {
        println!("\n✅ 测试通过！初始化和认证过程一致。");
        Ok(())
    } else {
        println!("\n❌ 测试失败！");
        println!("  正确密码验证: {}", is_valid);
        println!("  错误密码验证: {}", wrong_valid);
        Err("认证流程不一致".to_string())
    }
}