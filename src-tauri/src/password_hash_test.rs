// 密码哈希一致性测试
use sha2::{Digest, Sha256};

fn hash_password(password: &str, salt: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}

fn main() {
    println!("=== 密码哈希一致性测试 ===");
    
    let password = "test123456";
    let salt: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32
    ];
    
    println!("密码: {}", password);
    println!("盐值: {:?}", &salt[..8]);
    
    // 第一次计算哈希
    let hash1 = hash_password(password, &salt);
    println!("第一次哈希: {}...", &hash1[..16]);
    
    // 第二次计算哈希
    let hash2 = hash_password(password, &salt);
    println!("第二次哈希: {}...", &hash2[..16]);
    
    // 验证一致性
    if hash1 == hash2 {
        println!("✅ 哈希值一致");
    } else {
        println!("❌ 哈希值不一致");
        println!("  差异: {}", hash1 == hash2);
    }
    
    // 测试不同的盐值
    let salt2: [u8; 32] = [
        2, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32
    ];
    
    let hash3 = hash_password(password, &salt2);
    println!("不同盐值的哈希: {}...", &hash3[..16]);
    
    if hash1 != hash3 {
        println!("✅ 不同盐值产生不同哈希");
    } else {
        println!("❌ 相同哈希值（错误）");
    }
    
    // 测试不同的密码
    let password2 = "different123";
    let hash4 = hash_password(password2, &salt);
    println!("不同密码的哈希: {}...", &hash4[..16]);
    
    if hash1 != hash4 {
        println!("✅ 不同密码产生不同哈希");
    } else {
        println!("❌ 相同哈希值（错误）");
    }
}