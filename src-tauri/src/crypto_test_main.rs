use ssh_key_manager_lib::services::CryptoService;

fn main() {
    println!("=== 密码加密增强测试 ===");
    
    // 测试PBKDF2密钥派生
    println!("\n1. 测试PBKDF2密钥派生:");
    let password = "test123456";
    let salt = [1u8; 32];
    
    let key = CryptoService::derive_key(password, &salt);
    println!("派生密钥: {}...", &format!("{:x?}", &key[..8]));
    
    // 测试AES-256-GCM加密/解密
    println!("\n2. 测试AES-256-GCM加密/解密:");
    let mut crypto_service = CryptoService::new();
    crypto_service.set_master_key(password).unwrap();
    
    let test_data = "这是一段需要加密的测试数据";
    println!("原始数据: {}", test_data);
    
    let encrypted = crypto_service.encrypt(test_data.as_bytes()).unwrap();
    println!("加密后数据长度: {} 字节", encrypted.ciphertext.len());
    
    let decrypted = crypto_service.decrypt(&encrypted).unwrap();
    let decrypted_str = String::from_utf8(decrypted).unwrap();
    println!("解密后数据: {}", decrypted_str);
    
    if test_data == decrypted_str {
        println!("✅ 加密/解密测试通过");
    } else {
        println!("❌ 加密/解密测试失败");
    }
    
    // 测试密码验证
    println!("\n3. 测试密码验证:");
    if crypto_service.verify_password(password) {
        println!("✅ 正确密码验证通过");
    } else {
        println!("❌ 正确密码验证失败");
    }
    
    if !crypto_service.verify_password("wrong_password") {
        println!("✅ 错误密码正确拒绝");
    } else {
        println!("❌ 错误密码验证失败");
    }
    
    // 测试zeroize
    println!("\n4. 测试zeroize内存安全:");
    println!("CryptoService结构体实现了ZeroizeOnDrop特性");
    println!("主密钥在CryptoService实例被丢弃时会自动清零");
    
    println!("\n=== 测试完成 ===");
}