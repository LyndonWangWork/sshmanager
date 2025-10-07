use crate::services::{CryptoService, SshConfigService, SshKeyService};
use crate::storage::StorageService;
use crate::types::{KeyGenerationParams, SshKeyPair};
use base64::{engine::general_purpose, Engine as _};
use std::process::Command;
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

    // 添加新密钥（跳过已存在的 ID，并去重导入集合内部的重复）
    let keys_array = data["keys"].as_array_mut().ok_or("无效的数据格式")?;

    let mut existing_ids = std::collections::HashSet::new();
    for k in keys_array.iter() {
        if let Some(id) = k["id"].as_str() {
            existing_ids.insert(id.to_string());
        }
    }

    let mut seen_import_ids = std::collections::HashSet::new();
    let mut to_add: Vec<SshKeyPair> = Vec::new();
    for key in imported_keys.into_iter() {
        if existing_ids.contains(&key.id) {
            continue;
        }
        if !seen_import_ids.insert(key.id.clone()) {
            continue;
        }
        to_add.push(key);
    }

    for key in &to_add {
        keys_array.push(serde_json::to_value(key).map_err(|e| e.to_string())?);
    }

    // 保存数据
    save_encrypted_data(data, &crypto_state, &storage_state).await?;

    Ok(to_add)
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
                "is_encrypted": false,
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

    // 重置后保持未初始化状态：不写入新主密钥与任何数据
    // 此时存储文件为空，前端将进入初始化流程
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

// 验证主密码是否正确
#[tauri::command]
pub async fn verify_master_password(
    master_key: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<bool, String> {
    let mut crypto = crypto_state.lock().map_err(|e| e.to_string())?;
    let storage = storage_state.lock().map_err(|e| e.to_string())?;

    // 加载存储文件中的盐与哈希
    let (_encrypted_data, _salt_vec, stored_hash, salt) =
        storage.load_encrypted_data().map_err(|e| e.to_string())?;

    crypto.set_salt(salt);
    crypto.set_master_key_hash(stored_hash.clone());

    Ok(crypto.verify_password(&master_key))
}

// 导出所有密钥（加密JSON，使用用户提供的主密码）
#[tauri::command]
pub async fn export_all_keys_encrypted(
    master_key: String,
    include_private_keys: bool,
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

    let export_salt = crate::services::CryptoService::generate_salt();

    // 为每个密钥加密公钥与私钥
    let mut enc_keys: Vec<serde_json::Value> = Vec::new();
    for k in keys {
        let pub_enc = crate::services::CryptoService::encrypt_with_password(
            &master_key,
            &export_salt,
            k.public_key.as_bytes(),
        )
        .map_err(|e| e.to_string())?;

        let priv_enc_value = if include_private_keys {
            let priv_enc = crate::services::CryptoService::encrypt_with_password(
                &master_key,
                &export_salt,
                k.private_key.as_bytes(),
            )
            .map_err(|e| e.to_string())?;
            serde_json::json!({
                "nonce": general_purpose::STANDARD.encode(&priv_enc.nonce),
                "ciphertext": general_purpose::STANDARD.encode(&priv_enc.ciphertext),
            })
        } else {
            serde_json::Value::Null
        };

        let item = serde_json::json!({
            "id": k.id,
            "name": k.name,
            "key_type": k.key_type,
            "key_size": k.key_size,
            "comment": k.comment,
            "fingerprint": k.fingerprint,
            "created_at": k.created_at,
            "public_key_encrypted": {
                "nonce": general_purpose::STANDARD.encode(&pub_enc.nonce),
                "ciphertext": general_purpose::STANDARD.encode(&pub_enc.ciphertext)
            },
            "private_key_encrypted": priv_enc_value,
        });
        enc_keys.push(item);
    }

    let export_obj = serde_json::json!({
        "version": "1.1-encrypted",
        "is_encrypted": true,
        "exported_at": chrono::Utc::now(),
        "kdf": {
            "type": "pbkdf2-hmac-sha256",
            "iterations": 100_000,
        },
        "salt": general_purpose::STANDARD.encode(&export_salt),
        "keys": enc_keys,
    });

    serde_json::to_string_pretty(&export_obj).map_err(|e| format!("序列化失败: {}", e))
}

// 导出选中的密钥（加密JSON，使用用户提供的主密码）
#[tauri::command]
pub async fn export_selected_keys_encrypted(
    key_ids: Vec<String>,
    master_key: String,
    include_private_keys: bool,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<String, String> {
    let data = load_and_decrypt_data(&crypto_state, &storage_state).await?;

    let all_keys: Vec<SshKeyPair> = data["keys"]
        .as_array()
        .ok_or("无效的数据格式")?
        .iter()
        .map(|v| serde_json::from_value(v.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let keys: Vec<SshKeyPair> = if key_ids.is_empty() {
        all_keys
    } else {
        all_keys
            .into_iter()
            .filter(|k| key_ids.contains(&k.id))
            .collect()
    };

    if keys.is_empty() {
        return Err("没有找到要导出的密钥".to_string());
    }

    let export_salt = crate::services::CryptoService::generate_salt();
    let mut enc_keys: Vec<serde_json::Value> = Vec::new();
    for k in keys {
        let pub_enc = crate::services::CryptoService::encrypt_with_password(
            &master_key,
            &export_salt,
            k.public_key.as_bytes(),
        )
        .map_err(|e| e.to_string())?;

        let priv_enc_value = if include_private_keys {
            let priv_enc = crate::services::CryptoService::encrypt_with_password(
                &master_key,
                &export_salt,
                k.private_key.as_bytes(),
            )
            .map_err(|e| e.to_string())?;
            serde_json::json!({
                "nonce": general_purpose::STANDARD.encode(&priv_enc.nonce),
                "ciphertext": general_purpose::STANDARD.encode(&priv_enc.ciphertext),
            })
        } else {
            serde_json::Value::Null
        };

        let item = serde_json::json!({
            "id": k.id,
            "name": k.name,
            "key_type": k.key_type,
            "key_size": k.key_size,
            "comment": k.comment,
            "fingerprint": k.fingerprint,
            "created_at": k.created_at,
            "public_key_encrypted": {
                "nonce": general_purpose::STANDARD.encode(&pub_enc.nonce),
                "ciphertext": general_purpose::STANDARD.encode(&pub_enc.ciphertext)
            },
            "private_key_encrypted": priv_enc_value,
        });
        enc_keys.push(item);
    }

    let export_obj = serde_json::json!({
        "version": "1.1-encrypted",
        "is_encrypted": true,
        "exported_at": chrono::Utc::now(),
        "kdf": {
            "type": "pbkdf2-hmac-sha256",
            "iterations": 100_000,
        },
        "salt": general_purpose::STANDARD.encode(&export_salt),
        "keys": enc_keys,
    });

    serde_json::to_string_pretty(&export_obj).map_err(|e| format!("序列化失败: {}", e))
}

// 导入加密的密钥（使用用户主密码解密公钥/私钥）
#[tauri::command]
pub async fn import_encrypted_keys(
    keys_data: String,
    master_key: String,
    crypto_state: CryptoState<'_>,
    storage_state: StorageState<'_>,
) -> Result<Vec<SshKeyPair>, String> {
    let v: serde_json::Value =
        serde_json::from_str(&keys_data).map_err(|e| format!("解析密钥数据失败: {}", e))?;

    // 支持两种salt格式：base64字符串或字节数组
    let salt = if let Some(salt_str) = v.get("salt").and_then(|s| s.as_str()) {
        // 新格式：base64字符串
        let salt_vec = general_purpose::STANDARD
            .decode(salt_str)
            .map_err(|e| format!("salt base64解码失败: {}", e))?;

        if salt_vec.len() != 32 {
            return Err("salt长度无效".to_string());
        }

        let mut salt = [0u8; 32];
        salt.copy_from_slice(&salt_vec);
        salt
    } else if let Some(salt_vec) = v.get("salt").and_then(|s| s.as_array()) {
        // 旧格式：字节数组
        if salt_vec.len() != 32 {
            return Err("salt长度无效".to_string());
        }
        let mut salt = [0u8; 32];
        for (i, b) in salt_vec.iter().enumerate() {
            salt[i] = b.as_u64().ok_or("salt字节无效")? as u8;
        }
        salt
    } else {
        return Err("缺少salt或格式无效".to_string());
    };

    let items = v
        .get("keys")
        .and_then(|k| k.as_array())
        .ok_or("缺少keys或格式无效")?;

    let mut decrypted_keys: Vec<SshKeyPair> = Vec::new();
    for item in items {
        let id = item
            .get("id")
            .and_then(|x| x.as_str())
            .ok_or("缺少id")?
            .to_string();
        let name = item
            .get("name")
            .and_then(|x| x.as_str())
            .ok_or("缺少name")?
            .to_string();
        let key_type = {
            let s = item
                .get("key_type")
                .and_then(|x| x.as_str())
                .ok_or("缺少key_type")?;
            match s {
                "Rsa" | "RSA" | "rsa" => crate::types::SshKeyType::Rsa,
                "Ed25519" | "ed25519" => crate::types::SshKeyType::Ed25519,
                "Ecdsa" | "ECDSA" | "ecdsa" => crate::types::SshKeyType::Ecdsa,
                other => return Err(format!("不支持的key_type: {}", other)),
            }
        };
        let key_size = item
            .get("key_size")
            .and_then(|x| x.as_u64())
            .ok_or("缺少key_size")? as u32;
        let comment = item
            .get("comment")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();
        let fingerprint = item
            .get("fingerprint")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();

        let created_at = match item.get("created_at") {
            Some(x) if x.is_string() => chrono::DateTime::parse_from_rfc3339(x.as_str().unwrap())
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            Some(x) if x.is_object() => chrono::Utc::now(),
            _ => chrono::Utc::now(),
        };

        let pub_obj = item
            .get("public_key_encrypted")
            .ok_or("缺少public_key_encrypted")?;

        // 支持两种nonce格式：base64字符串或字节数组
        let pub_nonce_bytes = if let Some(nonce_str) = pub_obj.get("nonce").and_then(|a| a.as_str())
        {
            // 新格式：base64字符串
            general_purpose::STANDARD
                .decode(nonce_str)
                .map_err(|e| format!("public nonce base64解码失败: {}", e))?
        } else if let Some(nonce_array) = pub_obj.get("nonce").and_then(|a| a.as_array()) {
            // 旧格式：字节数组
            let mut nonce_bytes = Vec::with_capacity(nonce_array.len());
            for b in nonce_array {
                nonce_bytes.push(b.as_u64().ok_or("nonce字节无效")? as u8);
            }
            nonce_bytes
        } else {
            return Err("public_key_encrypted.nonce无效".to_string());
        };

        // 支持两种ciphertext格式：base64字符串或字节数组
        let pub_cipher_bytes = if let Some(cipher_str) =
            pub_obj.get("ciphertext").and_then(|a| a.as_str())
        {
            // 新格式：base64字符串
            general_purpose::STANDARD
                .decode(cipher_str)
                .map_err(|e| format!("public ciphertext base64解码失败: {}", e))?
        } else if let Some(cipher_array) = pub_obj.get("ciphertext").and_then(|a| a.as_array()) {
            // 旧格式：字节数组
            let mut cipher_bytes = Vec::with_capacity(cipher_array.len());
            for b in cipher_array {
                cipher_bytes.push(b.as_u64().ok_or("ciphertext字节无效")? as u8);
            }
            cipher_bytes
        } else {
            return Err("public_key_encrypted.ciphertext无效".to_string());
        };

        let pub_enc = crate::services::EncryptedData {
            nonce: pub_nonce_bytes,
            ciphertext: pub_cipher_bytes,
        };
        let public_key_bytes =
            crate::services::CryptoService::decrypt_with_password(&master_key, &salt, &pub_enc)
                .map_err(|_| "解密失败: 公钥".to_string())?;
        let public_key =
            String::from_utf8(public_key_bytes).map_err(|_| "公钥解码失败".to_string())?;

        let private_key = if let Some(priv_obj) = item.get("private_key_encrypted") {
            if priv_obj.is_null() {
                String::new()
            } else {
                // 支持两种nonce格式：base64字符串或字节数组
                let priv_nonce_bytes = if let Some(nonce_str) =
                    priv_obj.get("nonce").and_then(|a| a.as_str())
                {
                    // 新格式：base64字符串
                    general_purpose::STANDARD
                        .decode(nonce_str)
                        .map_err(|e| format!("private nonce base64解码失败: {}", e))?
                } else if let Some(nonce_array) = priv_obj.get("nonce").and_then(|a| a.as_array()) {
                    // 旧格式：字节数组
                    let mut nonce_bytes = Vec::with_capacity(nonce_array.len());
                    for b in nonce_array {
                        nonce_bytes.push(b.as_u64().ok_or("nonce字节无效")? as u8);
                    }
                    nonce_bytes
                } else {
                    return Err("private_key_encrypted.nonce无效".to_string());
                };

                // 支持两种ciphertext格式：base64字符串或字节数组
                let priv_cipher_bytes =
                    if let Some(cipher_str) = priv_obj.get("ciphertext").and_then(|a| a.as_str()) {
                        // 新格式：base64字符串
                        general_purpose::STANDARD
                            .decode(cipher_str)
                            .map_err(|e| format!("private ciphertext base64解码失败: {}", e))?
                    } else if let Some(cipher_array) =
                        priv_obj.get("ciphertext").and_then(|a| a.as_array())
                    {
                        // 旧格式：字节数组
                        let mut cipher_bytes = Vec::with_capacity(cipher_array.len());
                        for b in cipher_array {
                            cipher_bytes.push(b.as_u64().ok_or("ciphertext字节无效")? as u8);
                        }
                        cipher_bytes
                    } else {
                        return Err("private_key_encrypted.ciphertext无效".to_string());
                    };

                let priv_enc = crate::services::EncryptedData {
                    nonce: priv_nonce_bytes,
                    ciphertext: priv_cipher_bytes,
                };
                let private_key_bytes = crate::services::CryptoService::decrypt_with_password(
                    &master_key,
                    &salt,
                    &priv_enc,
                )
                .map_err(|_| "解密失败: 私钥".to_string())?;
                String::from_utf8(private_key_bytes).map_err(|_| "私钥解码失败".to_string())?
            }
        } else {
            String::new()
        };

        decrypted_keys.push(SshKeyPair {
            id,
            name,
            key_type,
            key_size,
            comment,
            fingerprint,
            public_key,
            private_key,
            created_at,
            last_used: None,
        });
    }

    // 加载现有数据
    let mut data = load_and_decrypt_data(&crypto_state, &storage_state).await?;

    // 添加新密钥（跳过已存在的 ID，并去重导入集合内部的重复）
    let keys_array = data["keys"].as_array_mut().ok_or("无效的数据格式")?;

    let mut existing_ids = std::collections::HashSet::new();
    for k in keys_array.iter() {
        if let Some(id) = k["id"].as_str() {
            existing_ids.insert(id.to_string());
        }
    }

    let mut seen_import_ids = std::collections::HashSet::new();
    let mut to_add: Vec<SshKeyPair> = Vec::new();
    for key in decrypted_keys.into_iter() {
        if existing_ids.contains(&key.id) {
            continue;
        }
        if !seen_import_ids.insert(key.id.clone()) {
            continue;
        }
        to_add.push(key);
    }

    for key in &to_add {
        keys_array.push(serde_json::to_value(key).map_err(|e| e.to_string())?);
    }

    // 保存数据
    save_encrypted_data(data, &crypto_state, &storage_state).await?;

    Ok(to_add)
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

// 确保目录存在
#[tauri::command]
pub async fn ensure_dir_exists(dir_path: String) -> Result<bool, String> {
    std::fs::create_dir_all(&dir_path)
        .map(|_| true)
        .map_err(|e| format!("创建目录失败: {}", e))
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

// 检查文件是否存在
#[tauri::command]
pub async fn check_file_exists(file_path: String) -> Result<bool, String> {
    Ok(std::path::Path::new(&file_path).exists())
}

// 打开配置目录 ~/.ssh
#[tauri::command]
pub async fn open_config_directory(_app: tauri::AppHandle) -> Result<bool, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法获取用户主目录".to_string())?;
    let ssh_dir = home.join(".ssh");
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(ssh_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(ssh_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(ssh_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }
}

// 打开任意目录（跨平台）
#[tauri::command]
pub async fn open_directory(dir_path: String) -> Result<bool, String> {
    if dir_path.trim().is_empty() {
        return Err("目录路径为空".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(dir_path)
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(dir_path)
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(dir_path)
            .spawn()
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }
}

// 备份导出目录中的导出文件：在重置应用前调用
#[tauri::command]
pub async fn backup_export_files(dir_path: String) -> Result<bool, String> {
    use std::fs;
    use std::path::{Path, PathBuf};

    if dir_path.trim().is_empty() {
        return Err("目录路径为空".to_string());
    }

    let base = Path::new(&dir_path);
    if !base.exists() {
        // 导出目录不存在则视为无文件可备份，直接返回成功
        return Ok(true);
    }

    let backup_dir: PathBuf = base.join("backup");
    fs::create_dir_all(&backup_dir).map_err(|e| format!("创建备份目录失败: {}", e))?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();

    for entry in fs::read_dir(base).map_err(|e| format!("读取目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        // 跳过目录与备份目录本身
        if path.is_dir() {
            continue;
        }

        // 仅处理文件
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                // 生成带时间戳的新文件名
                let (stem, ext) = match (
                    path.file_stem().and_then(|s| s.to_str()),
                    path.extension().and_then(|e| e.to_str()),
                ) {
                    (Some(stem), Some(ext)) => (stem.to_string(), Some(ext.to_string())),
                    (Some(stem), None) => (stem.to_string(), None),
                    _ => (file_name.to_string(), None),
                };

                let new_name = match ext {
                    Some(ext) => format!("{}_{}.{}", stem, timestamp, ext),
                    None => format!("{}_{}", stem, timestamp),
                };

                let target = backup_dir.join(new_name);
                fs::copy(&path, &target)
                    .map_err(|e| format!("备份文件失败 ({}): {}", file_name, e))?;
            }
        }
    }

    Ok(true)
}
