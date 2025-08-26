use tauri::State;
use std::sync::Mutex;
use crate::services::{CryptoService, SshKeyService};
use crate::types::{SshKeyPair, KeyGenerationParams};
use crate::error::{AppError, AppResult};
use crate::storage::StorageService;

type CryptoState<'a> = State<'a, Mutex<CryptoService>>;
type StorageState<'a> = State<'a, Mutex<StorageService>>;

// 检查是否已初始化
#[tauri::command]
pub async fn is_initialized(storage: StorageState<'_>) -> Result<bool, String> {
    let storage = storage.lock().map_err(|e| e.to_string())?;
    Ok(storage.is_initialized())
}

// 初始化应用
#[tauri::command]
pub async fn initialize_app(
    master_key: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let mut crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let mut storage = storage_state.lock().map_err(|e| e.to_string())?;
    
    // 生成随机盐值
    let salt = CryptoService::generate_salt();
    
    // 设置主密钥
    crypto.set_master_key(&master_key, &salt);
    
    // 创建初始存储
    let initial_data = serde_json::json!({
        "keys": [],
        "config": {
            "theme": "light",
            "auto_backup": true,
            "backup_retention": 10,
            "default_key_type": "Ed25519",
            "default_key_size": 256
        }
    });
    
    // 加密并保存
    let encrypted = crypto.encrypt(initial_data.to_string().as_bytes())
        .map_err(|e| e.to_string())?;
    
    // 保存到本地文件
    storage.save_encrypted_data(&encrypted, &salt).map_err(|e| e.to_string())?;
    
    Ok(true)
}

// 用户认证
#[tauri::command]
pub async fn authenticate(
    master_key: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let mut crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let storage = storage_state.lock().map_err(|e| e.to_string())?;
    
    // 加载存储文件
    let (encrypted_data, salt) = storage.load_encrypted_data().map_err(|e| e.to_string())?;
    
    // 设置主密钥
    crypto.set_master_key(&master_key, &salt);
    
    // 尝试解密来验证密码
    match crypto.decrypt(&encrypted_data) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

// 生成SSH密钥
#[tauri::command]
pub async fn generate_ssh_key(
    params: KeyGenerationParams,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<SshKeyPair, String> {
    // 生成密钥对
    let key_pair = SshKeyService::generate_key_pair(params)
        .map_err(|e| e.to_string())?;
    
    // 加载现有数据
    let mut data = load_and_decrypt_data(&crypto_state, &storage_state).await?;
    
    // 添加新密钥
    data["keys"].as_array_mut()
        .ok_or("无效的数据格式")?
        .push(serde_json::to_value(&key_pair).map_err(|e| e.to_string())?);
    
    // 保存数据
    save_encrypted_data(data, &crypto_state, &storage_state).await?;
    
    Ok(key_pair)
}

// 获取所有密钥
#[tauri::command]
pub async fn get_all_keys(
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<Vec<SshKeyPair>, String> {
    let data = load_and_decrypt_data(&crypto_state, &storage_state).await?;
    
    let keys: Vec<SshKeyPair> = data["keys"]
        .as_array()
        .ok_or("无效的数据格式")?
        .iter()
        .map(|v| serde_json::from_value(v.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(keys)
}

// 删除密钥
#[tauri::command]
pub async fn delete_key(
    key_id: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let mut data = load_and_decrypt_data(&crypto_state, &storage_state).await?;
    
    let keys = data["keys"].as_array_mut()
        .ok_or("无效的数据格式")?;
    
    // 查找并删除密钥
    let initial_len = keys.len();
    keys.retain(|key| {
        key["id"].as_str().unwrap_or("") != key_id
    });
    
    if keys.len() < initial_len {
        save_encrypted_data(data, &crypto_state, &storage_state).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

// 导出密钥（实现版本）
#[tauri::command]
pub async fn export_key(
    key_id: String,
    export_path: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let data = load_and_decrypt_data(&crypto_state, &storage_state).await?;
    
    // 查找密钥
    let keys: Vec<serde_json::Value> = data["keys"]
        .as_array()
        .ok_or("无效的数据格式")?
        .iter()
        .filter(|k| k["id"].as_str().unwrap_or("") == key_id)
        .cloned()
        .collect();
    
    if keys.is_empty() {
        return Err("密钥不存在".to_string());
    }
    
    let key = &keys[0];
    
    // 导出公钥文件
    let public_key_content = key["public_key"].as_str().unwrap_or("");
    let public_key_path = format!("{}.pub", export_path);
    
    std::fs::write(&public_key_path, public_key_content)
        .map_err(|e| format!("写入公钥文件失败: {}", e))?;
    
    // 导出私钥文件 (注意：实际应用中需要解密)
    let private_key_content = key["private_key"].as_str().unwrap_or("");
    std::fs::write(&export_path, private_key_content)
        .map_err(|e| format!("写入私钥文件失败: {}", e))?;
    
    Ok(true)
}

// 导入密钥
#[tauri::command]
pub async fn import_keys(
    keys_data: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<Vec<SshKeyPair>, String> {
    // 解析导入的密钥数据
    let imported_keys: Vec<SshKeyPair> = serde_json::from_str(&keys_data)
        .map_err(|e| format!("解析密钥数据失败: {}", e))?;
    
    // 加载现有数据
    let mut data = load_and_decrypt_data(&crypto_state, &storage_state).await?;
    
    // 添加新密钥
    let keys_array = data["keys"].as_array_mut()
        .ok_or("无效的数据格式")?;
    
    for key in &imported_keys {
        keys_array.push(serde_json::to_value(key).map_err(|e| e.to_string())?);
    }
    
    // 保存数据
    save_encrypted_data(data, &crypto_state, &storage_state).await?;
    
    Ok(imported_keys)
}

// 导出所有密钥
#[tauri::command]
pub async fn export_all_keys(
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<String, String> {
    let data = load_and_decrypt_data(&crypto_state, &storage_state).await?;
    
    let keys: Vec<SshKeyPair> = data["keys"]
        .as_array()
        .ok_or("无效的数据格式")?
        .iter()
        .map(|v| serde_json::from_value(v.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    // 创建导出数据结构
    let export_data = serde_json::json!({
        "version": "1.0",
        "exported_at": chrono::Utc::now(),
        "keys": keys
    });
    
    serde_json::to_string_pretty(&export_data)
        .map_err(|e| format!("序列化失败: {}", e))
}

// 辅助函数
async fn load_and_decrypt_data(
    crypto_state: &CryptoState<'_>,
    storage_state: &StorageState<'_>,
) -> Result<serde_json::Value, String> {
    let crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let storage = storage_state.lock().map_err(|e| e.to_string())?;
    
    let (encrypted_data, _) = storage.load_encrypted_data().map_err(|e| e.to_string())?;
    
    let decrypted = crypto.decrypt(&encrypted_data).map_err(|e| e.to_string())?;
    let data_str = String::from_utf8(decrypted).map_err(|e| e.to_string())?;
    serde_json::from_str(&data_str).map_err(|e| e.to_string())
}

async fn save_encrypted_data(
    data: serde_json::Value,
    crypto_state: &CryptoState<'_>,
    storage_state: &StorageState<'_>,
) -> Result<(), String> {
    let crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let mut storage = storage_state.lock().map_err(|e| e.to_string())?;
    
    let data_str = serde_json::to_string(&data).map_err(|e| e.to_string())?;
    
    let encrypted = crypto.encrypt(data_str.as_bytes()).map_err(|e| e.to_string())?;
    let salt = CryptoService::generate_salt(); // 在实际实现中应使用现有盐值
    
    storage.save_encrypted_data(&encrypted, &salt).map_err(|e| e.to_string())
}