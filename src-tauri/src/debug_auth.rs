// 调试程序：验证密码处理过程
use sha2::{Digest, Sha256};
use rand::RngCore;

fn hash_password(password: &str, salt: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}

fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

fn main() {
    println!("=== 密码处理过程调试 ===");
    
    let password = "test123456";
    println!("原始密码: {}", password);
    
    // 1. 初始化过程
    println!("\n--- 初始化过程 ---");
    let salt = generate_salt();
    println!("生成的盐值: {:?}", &salt[..8]); // 只显示前8个字节
    
    let hash = hash_password(password, &salt);
    println!("生成的哈希值: {}...", &hash[..16]); // 只显示前16个字符
    
    // 2. 模拟保存过程（转换为JSON数组再读取）
    println!("\n--- 模拟保存和读取过程 ---");
    let salt_json: Vec<serde_json::Value> = salt.iter().map(|&b| serde_json::Value::Number(b.into())).collect();
    println!("盐值转换为JSON数组长度: {}", salt_json.len());
    
    // 模拟从JSON数组读取盐值
    let salt_vec: Result<Vec<u8>, _> = salt_json
        .iter()
        .map(|v| v.as_u64().ok_or("错误").map(|u| u as u8))
        .collect();
    
    let salt_vec = salt_vec.unwrap();
    println!("从JSON数组读取的盐值长度: {}", salt_vec.len());
    
    let mut restored_salt = [0u8; 32];
    restored_salt.copy_from_slice(&salt_vec);
    println!("恢复的盐值是否与原盐值相同: {}", restored_salt == salt);
    
    // 3. 验证过程
    println!("\n--- 验证过程 ---");
    let verify_hash = hash_password(password, &restored_salt);
    println!("验证时生成的哈希值: {}...", &verify_hash[..16]); // 只显示前16个字符
    println!("哈希值是否匹配: {}", verify_hash == hash);
    
    // 4. 错误密码测试
    let wrong_hash = hash_password("wrongpassword", &restored_salt);
    println!("错误密码生成的哈希值: {}...", &wrong_hash[..16]); // 只显示前16个字符
    println!("错误密码哈希值是否匹配: {}", wrong_hash == hash);
    
    println!("\n=== 调试完成 ===");
}