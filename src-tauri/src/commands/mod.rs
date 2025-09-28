use crate::services::{CryptoService, SshConfigService, SshKeyService};
use crate::storage::StorageService;
use crate::types::{KeyGenerationParams, SshKeyPair};
use std::sync::Mutex;
use tauri::State;

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

    // 设置主密钥（这会生成盐值和派生密钥）
    crypto
        .set_master_key(&master_key)
        .map_err(|e| e.to_string())?;

    // 验证密码设置成功
    if !crypto.verify_password(&master_key) {
        return Err("密码验证失败".to_string());
    }

    // 获取盐值和哈希
    let salt = crypto.get_salt().unwrap();
    let master_key_hash = crypto.get_master_key_hash().unwrap();

    // 创建初始存储（注意：不将master_key_hash和salt存储在加密数据中）
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
    let encrypted = crypto
        .encrypt(initial_data.to_string().as_bytes())
        .map_err(|e| e.to_string())?;

    // 保存到本地文件（master_key_hash和salt作为元数据存储）
    storage
        .save_encrypted_data(&encrypted, &salt, &master_key_hash)
        .map_err(|e| e.to_string())?;

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
    let (_encrypted_data, _salt_vec, stored_hash, salt) =
        storage.load_encrypted_data().map_err(|e| e.to_string())?;

    // 设置盐值和存储的主密码哈希
    crypto.set_salt(salt);
    crypto.set_master_key_hash(stored_hash.clone());

    // 验证密码
    let is_valid = crypto.verify_password(&master_key);

    // 如果密码验证成功，设置主密钥（使用已有的盐值派生密钥，而不是生成新的盐值）
    if is_valid {
        let derived_key = CryptoService::derive_key(&master_key, &salt);
        crypto.set_derived_master_key(derived_key);
    }

    Ok(is_valid)
}

// 生成SSH密钥
#[tauri::command]
pub async fn generate_ssh_key(
    params: KeyGenerationParams,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<SshKeyPair, String> {
    // 生成密钥对
    let key_pair = SshKeyService::generate_key_pair(params).map_err(|e| e.to_string())?;

    // 加载现有数据
    let mut data = load_and_decrypt_data(&crypto_state, &storage_state).await?;

    // 添加新密钥
    data["keys"]
        .as_array_mut()
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

    let keys = data["keys"].as_array_mut().ok_or("无效的数据格式")?;

    // 查找并删除密钥
    let initial_len = keys.len();
    keys.retain(|key| key["id"].as_str().unwrap_or("") != key_id);

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
    let imported_keys: Vec<SshKeyPair> =
        serde_json::from_str(&keys_data).map_err(|e| format!("解析密钥数据失败: {}", e))?;

    // 加载现有数据
    let mut data = load_and_decrypt_data(&crypto_state, &storage_state).await?;

    // 添加新密钥
    let keys_array = data["keys"].as_array_mut().ok_or("无效的数据格式")?;

    for key in &imported_keys {
        keys_array.push(serde_json::to_value(key).map_err(|e| e.to_string())?);
    }

    // 保存数据
    save_encrypted_data(data, &crypto_state, &storage_state).await?;

    Ok(imported_keys)
}

// 导出密钥到指定文件（增强版本）
#[tauri::command]
pub async fn export_keys_to_file(
    key_ids: Vec<String>,
    file_path: String,
    export_format: String,
    include_private_keys: bool,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let data = load_and_decrypt_data(&crypto_state, &storage_state).await?;

    // 获取所有密钥
    let all_keys: Vec<SshKeyPair> = data["keys"]
        .as_array()
        .ok_or("无效的数据格式")?
        .iter()
        .map(|v| serde_json::from_value(v.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // 筛选要导出的密钥
    let keys_to_export: Vec<SshKeyPair> = if key_ids.is_empty() {
        // 如果没有指定密钥ID，导出所有密钥
        all_keys
    } else {
        // 根据ID筛选密钥
        all_keys
            .into_iter()
            .filter(|key| key_ids.contains(&key.id))
            .collect()
    };

    if keys_to_export.is_empty() {
        return Err("没有找到要导出的密钥".to_string());
    }

    // 根据格式导出
    match export_format.as_str() {
        "json" => {
            let export_data = serde_json::json!({
                "version": "1.0",
                "exported_at": chrono::Utc::now(),
                "keys": keys_to_export.iter().map(|key| {
                    let mut key_data = serde_json::to_value(key).unwrap();
                    if !include_private_keys {
                        key_data["private_key"] = serde_json::Value::String("[REDACTED]".to_string());
                    }
                    key_data
                }).collect::<Vec<_>>()
            });

            let json_content = serde_json::to_string_pretty(&export_data)
                .map_err(|e| format!("序列化失败: {}", e))?;

            std::fs::write(&file_path, json_content).map_err(|e| format!("写入文件失败: {}", e))?;
        }
        "openssh" => {
            // 导出为OpenSSH格式（每个密钥一个文件）
            for (index, key) in keys_to_export.iter().enumerate() {
                let base_path = if keys_to_export.len() == 1 {
                    file_path.clone()
                } else {
                    format!("{}_key_{}", file_path, index + 1)
                };

                // 写入公钥文件
                let pub_path = format!("{}.pub", base_path);
                std::fs::write(&pub_path, &key.public_key)
                    .map_err(|e| format!("写入公钥文件失败: {}", e))?;

                // 如果包含私钥，写入私钥文件
                if include_private_keys {
                    std::fs::write(&base_path, &key.private_key)
                        .map_err(|e| format!("写入私钥文件失败: {}", e))?;
                }
            }
        }
        "pem" => {
            // 导出为PEM格式
            let mut pem_content = String::new();
            for key in &keys_to_export {
                pem_content.push_str(&format!("# SSH Key: {}\n", key.name));
                pem_content.push_str(&format!("# Type: {}\n", key.key_type));
                pem_content.push_str(&format!("# Size: {} bits\n", key.key_size));
                pem_content.push_str(&format!("# Fingerprint: {}\n", key.fingerprint));
                pem_content.push_str(&format!(
                    "# Created: {}\n",
                    key.created_at.format("%Y-%m-%d %H:%M:%S UTC")
                ));
                if !key.comment.is_empty() {
                    pem_content.push_str(&format!("# Comment: {}\n", key.comment));
                }
                pem_content.push_str("#\n");
                pem_content.push_str(&format!("# Public Key:\n{}\n\n", key.public_key));

                if include_private_keys {
                    pem_content.push_str(&format!("# Private Key:\n{}\n\n", key.private_key));
                } else {
                    pem_content.push_str("# Private Key: [REDACTED for security]\n\n");
                }

                pem_content.push_str(&format!("# {}\n\n", "-".repeat(50)));
            }

            std::fs::write(&file_path, pem_content).map_err(|e| format!("写入文件失败: {}", e))?;
        }
        _ => {
            return Err(format!("不支持的导出格式: {}", export_format));
        }
    }

    Ok(true)
}

// 重置所有数据
#[tauri::command]
pub async fn reset_all_data(
    master_key: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let mut crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let mut storage = storage_state.lock().map_err(|e| e.to_string())?;

    // 首先验证密码
    // 加载存储文件
    let (_encrypted_data, _salt_vec, stored_hash, salt) =
        storage.load_encrypted_data().map_err(|e| e.to_string())?;

    // 设置盐值和存储的主密码哈希
    crypto.set_salt(salt);
    crypto.set_master_key_hash(stored_hash.clone());

    // 验证密码
    let is_valid = crypto.verify_password(&master_key);

    if !is_valid {
        return Ok(false);
    }

    // 清除加密服务中的主密钥
    crypto.clear_master_key();

    // 清除存储服务中的所有数据
    storage.reset_storage().map_err(|e| e.to_string())?;

    // 生成新的盐值
    let salt = CryptoService::generate_salt();

    // 创建默认配置
    let default_config = serde_json::json!({
        "theme": "light",
        "auto_backup": true,
        "backup_retention": 10,
        "default_key_type": "Ed25519",
        "default_key_size": 256
    });

    // 创建初始数据
    let initial_data = serde_json::json!({
        "keys": [],
        "config": default_config
    });

    // 不使用加密服务加密，而是直接创建未加密的初始数据结构
    let storage_data = crate::types::EncryptedStorage {
        version: "1.0".to_string(),
        salt: salt.to_vec(),
        master_key_hash: "".to_string(),
        iv: vec![],                                                   // 空的初始化向量
        encrypted_data: initial_data.to_string().as_bytes().to_vec(), // 直接存储数据
        checksum: "".to_string(),                                     // 空校验和
        data: serde_json::Map::new(),                                 // 空的data字段
    };

    // 直接保存到存储，不使用加密服务
    let serialized = serde_json::to_string(&storage_data).map_err(|e| e.to_string())?;
    std::fs::write(&storage.storage_path(), serialized).map_err(|e| e.to_string())?;

    // 设置默认认证状态
    crypto.set_master_key_hash("".to_string());
    crypto.set_salt(salt);

    Ok(true)
}

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

    serde_json::to_string_pretty(&export_data).map_err(|e| format!("序列化失败: {}", e))
}

// 辅助函数
async fn load_and_decrypt_data(
    crypto_state: &CryptoState<'_>,
    storage_state: &StorageState<'_>,
) -> Result<serde_json::Value, String> {
    let crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let storage = storage_state.lock().map_err(|e| e.to_string())?;

    let (encrypted_data, _salt_vec, _stored_hash, _salt) =
        storage.load_encrypted_data().map_err(|e| e.to_string())?;

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

    let encrypted = crypto
        .encrypt(data_str.as_bytes())
        .map_err(|e| e.to_string())?;
    let salt = crypto.get_salt().ok_or("盐值未设置".to_string())?;
    let master_key_hash = crypto
        .get_master_key_hash()
        .ok_or("主密钥哈希未设置".to_string())?;

    storage
        .save_encrypted_data(&encrypted, &salt, &master_key_hash)
        .map_err(|e| e.to_string())
}

// 字符串格式化测试
#[cfg(test)]
mod string_formatting_tests {

    #[test]
    fn test_string_concatenation() {
        // 测试正确的字符串拼接方法
        let separator = "-".repeat(50);

        // 方法1: 使用format!宏
        let result1 = format!("# {}\n\n", separator);
        assert!(result1.starts_with("# "));
        assert!(result1.ends_with("\n\n"));
        assert!(result1.contains("---"));

        // 方法2: 使用push_str
        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&separator);
        content.push_str("\n\n");

        assert_eq!(result1, content);

        // 验证分隔符长度
        assert_eq!(separator.len(), 50);
    }

    #[test]
    fn test_pem_content_formatting() {
        // 模拟PEM内容格式化
        let mut pem_content = String::new();

        // 添加密钥信息
        pem_content.push_str("# SSH Key: test-key\n");
        pem_content.push_str("# Type: Ed25519\n");
        pem_content.push_str("#\n");
        pem_content.push_str("# Public Key:\nssh-ed25519 AAAAC3...\n\n");
        pem_content.push_str("# Private Key: [REDACTED for security]\n\n");

        // 添加分隔符 - 使用正确的方法
        pem_content.push_str(&format!("# {}\n\n", "-".repeat(50)));

        // 验证内容
        assert!(pem_content.contains("# SSH Key: test-key"));
        assert!(pem_content.contains("# Type: Ed25519"));
        assert!(pem_content.contains("[REDACTED for security]"));
        assert!(pem_content.contains(&"-".repeat(50)));

        // 验证格式正确
        let lines: Vec<&str> = pem_content.lines().collect();
        assert!(lines.iter().any(|line| line.starts_with("# SSH Key:")));
        assert!(lines.iter().any(|line| line.starts_with("# Type:")));
    }
}

// 简单的文件写入命令（用于前端直接保存文件）
#[tauri::command]
pub async fn write_file_content(file_path: String, content: String) -> Result<bool, String> {
    std::fs::write(&file_path, content)
        .map(|_| true)
        .map_err(|e| format!("文件写入失败: {}", e))
}

// 保存系统 SSH 配置（带备份与保留）
#[tauri::command]
pub async fn save_ssh_config(
    content: String,
    file_path: Option<String>,
    retention: Option<usize>,
) -> Result<bool, String> {
    SshConfigService::save_config(&content, file_path.as_deref(), retention)
        .map(|_| true)
        .map_err(|e| e.to_string())
}

// 读取并解析系统 SSH 配置
#[tauri::command]
pub async fn read_ssh_config(file_path: Option<String>) -> Result<crate::types::SshConfig, String> {
    SshConfigService::read_config(file_path.as_deref()).map_err(|e| e.to_string())
}

// 列出 ~/.ssh 目录下的私钥文件
#[tauri::command]
pub async fn list_identity_files(dir_path: Option<String>) -> Result<Vec<String>, String> {
    SshConfigService::list_identity_files(dir_path.as_deref()).map_err(|e| e.to_string())
}
