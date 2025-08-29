// 读取存储文件内容
use std::fs;
use std::path::PathBuf;
use dirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取存储文件路径
    let app_dir = dirs::config_dir()
        .ok_or("无法获取配置目录")?
        .join("sshmanager");
    
    let storage_path = app_dir.join("storage.enc");
    println!("存储文件路径: {:?}", storage_path);
    
    // 检查文件是否存在
    if !storage_path.exists() {
        println!("存储文件不存在");
        return Ok(());
    }
    
    // 读取文件内容
    let content = fs::read_to_string(&storage_path)?;
    println!("文件内容:");
    println!("{}", content);
    
    // 尝试解析JSON
    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(json) => {
            println!("\n解析后的JSON结构:");
            if let Some(data) = json.get("data") {
                println!("data字段:");
                println!("{}", serde_json::to_string_pretty(data)?);
            }
            
            if let Some(master_key_hash) = json.get("data").and_then(|d| d.get("master_key_hash")) {
                println!("\nmaster_key_hash字段:");
                println!("{}", serde_json::to_string_pretty(master_key_hash)?);
            }
            
            if let Some(salt) = json.get("data").and_then(|d| d.get("salt")) {
                println!("\nsalt字段:");
                println!("{}", serde_json::to_string_pretty(salt)?);
            }
        }
        Err(e) => {
            println!("JSON解析失败: {}", e);
        }
    }
    
    Ok(())
}