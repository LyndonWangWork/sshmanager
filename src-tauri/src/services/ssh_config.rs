use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::Local;

use crate::error::{AppError, AppResult};
use crate::types::{SshConfig, SshHostConfig};

pub struct SshConfigService;

impl SshConfigService {
    /// 保存 SSH 配置到目标路径。在写入前，如果目标文件已存在，将创建带时间戳的备份。
    /// 在备份数量超过保留值时，清理最旧的备份。
    pub fn save_config(
        content: &str,
        file_path: Option<&str>,
        retention: Option<usize>,
    ) -> AppResult<()> {
        let retention = retention.unwrap_or(10);

        let target_path = match file_path {
            Some(p) if !p.trim().is_empty() => PathBuf::from(p),
            _ => default_ssh_config_path()?,
        };

        // 确保目录存在
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 写入前做备份（如果存在）
        if target_path.exists() {
            create_backup(&target_path)?;
            enforce_retention(&target_path, retention)?;
        }

        // 写入新内容
        fs::write(&target_path, content)?;
        Ok(())
    }

    /// 读取 SSH 配置并解析为结构化数据
    pub fn read_config(file_path: Option<&str>) -> AppResult<SshConfig> {
        let target_path = match file_path {
            Some(p) if !p.trim().is_empty() => PathBuf::from(p),
            _ => default_ssh_config_path()?,
        };

        // 如果默认路径不存在，尝试 config.txt 作为兼容（Windows 常见）
        let path_to_read = if !target_path.exists() {
            let txt = target_path.with_extension("txt");
            if txt.exists() {
                txt
            } else {
                target_path
            }
        } else {
            target_path
        };

        let content = if path_to_read.exists() {
            fs::read_to_string(&path_to_read)?
        } else {
            String::new()
        };

        Ok(parse_openssh_config(&content))
    }
}

fn default_ssh_dir() -> AppResult<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| AppError::ConfigError("无法获取用户主目录".to_string()))?;
    Ok(home.join(".ssh"))
}

impl SshConfigService {
    /// 列出 ~/.ssh 目录下可能的私钥文件
    /// 规则：
    /// - 常见私钥名称（id_rsa/id_ecdsa/id_ed25519/id_dsa）
    /// - 任意不以 .pub 结尾的文件，且内容前几百字节包含 "PRIVATE KEY"（尽量避免误报）
    pub fn list_identity_files(dir_path: Option<&str>) -> AppResult<Vec<String>> {
        let dir = match dir_path {
            Some(p) if !p.trim().is_empty() => PathBuf::from(p),
            _ => default_ssh_dir()?,
        };

        if !dir.exists() || !dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut result: Vec<String> = Vec::new();
        let common_names = ["id_rsa", "id_ecdsa", "id_ed25519", "id_dsa"];

        for entry in fs::read_dir(&dir)? {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let file_name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };

            // 跳过公钥和明显的备份文件
            if file_name.ends_with(".pub")
                || file_name.ends_with(".bak")
                || file_name.ends_with(".backup")
            {
                continue;
            }

            // 常见私钥名直接包含
            let mut include = common_names.contains(&file_name.as_str());

            // 否则做一次轻量内容检查
            if !include {
                if let Ok(mut f) = fs::File::open(&path) {
                    use std::io::Read;
                    let mut buf = [0u8; 512];
                    let _ = f.read(&mut buf);
                    let head = String::from_utf8_lossy(&buf);
                    if head.contains("PRIVATE KEY") {
                        include = true;
                    }
                }
            }

            if include {
                result.push(file_name);
            }
        }

        // 排序保证稳定性
        result.sort();
        result.dedup();
        Ok(result)
    }
}

fn default_ssh_config_path() -> AppResult<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| AppError::ConfigError("无法获取用户主目录".to_string()))?;
    Ok(home.join(".ssh").join("config"))
}

/// 极简 OpenSSH config 解析器，提取全局设置与 Host 块常见字段
fn parse_openssh_config(content: &str) -> SshConfig {
    let mut global_settings: HashMap<String, String> = HashMap::new();
    let mut hosts: Vec<SshHostConfig> = Vec::new();

    let mut current_host: Option<SshHostConfig> = None;

    for raw_line in content.lines() {
        let mut line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }

        // 去除行内注释（简单处理：按 # 切割）
        if let Some(idx) = line.find('#') {
            line = &line[..idx].trim();
            if line.is_empty() {
                continue;
            }
        }

        // 按空白分割 key 与 value
        let mut parts = line.split_whitespace();
        let key = match parts.next() {
            Some(k) => k,
            None => continue,
        };
        let value = parts.collect::<Vec<_>>().join(" ").trim().to_string();
        if value.is_empty() {
            continue;
        }

        let key_lc = key.to_lowercase();
        if key_lc == "host" {
            // 推入上一个 host
            if let Some(h) = current_host.take() {
                hosts.push(h);
            }

            let host_pattern = value; // 允许包含空格（多个模式），此处整体保留
            current_host = Some(SshHostConfig {
                host_pattern,
                hostname: None,
                user: None,
                port: None,
                identity_file: None,
                other_options: HashMap::new(),
            });
            continue;
        }

        // 普通键值对
        if let Some(h) = current_host.as_mut() {
            match key_lc.as_str() {
                "hostname" => h.hostname = Some(value),
                "user" => h.user = Some(value),
                "port" => {
                    if let Ok(p) = value.parse::<u16>() {
                        h.port = Some(p);
                    }
                }
                "identityfile" | "identity_file" => h.identity_file = Some(value),
                _ => {
                    if !key_lc.is_empty() {
                        h.other_options.insert(key.to_string(), value);
                    }
                }
            }
        } else {
            // 全局设置
            if !key_lc.is_empty() {
                global_settings.insert(key.to_string(), value);
            }
        }
    }

    if let Some(h) = current_host.take() {
        hosts.push(h);
    }

    SshConfig {
        hosts,
        global_settings,
    }
}

fn create_backup(target_path: &Path) -> AppResult<PathBuf> {
    let parent = target_path
        .parent()
        .ok_or_else(|| AppError::ConfigError("无效的配置路径（缺少父目录）".to_string()))?;

    let ts = Local::now().format("%Y%m%d%H%M%S"); // 避免 Windows 冒号字符
    let backup_name = format!(
        "{}.bak.{}",
        target_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AppError::ConfigError("无法解析文件名".to_string()))?,
        ts
    );
    let backup_path = parent.join(backup_name);

    fs::copy(target_path, &backup_path)?;
    Ok(backup_path)
}

fn enforce_retention(target_path: &Path, retention: usize) -> AppResult<()> {
    if retention == 0 {
        // 如果保留为0，删除所有现有备份
        remove_all_backups(target_path)?;
        return Ok(());
    }

    let parent = target_path
        .parent()
        .ok_or_else(|| AppError::ConfigError("无效的配置路径（缺少父目录）".to_string()))?;

    let file_stem = target_path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| AppError::ConfigError("无法解析文件名".to_string()))?;

    // 列出形如 <file>.bak.<timestamp> 的备份
    let mut backups: Vec<PathBuf> = fs::read_dir(parent)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                name.starts_with(&format!("{}{}", file_stem, ".bak."))
            } else {
                false
            }
        })
        .collect();

    // 按文件名（包含时间戳）排序，新的在后
    backups.sort();

    if backups.len() > retention {
        let to_remove = backups.len() - retention;
        for old in backups.into_iter().take(to_remove) {
            let _ = fs::remove_file(old);
        }
    }
    Ok(())
}

fn remove_all_backups(target_path: &Path) -> io::Result<()> {
    if let Some(parent) = target_path.parent() {
        let file_stem = target_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        for entry in fs::read_dir(parent)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name.starts_with(&format!("{}{}", file_stem, ".bak.")) {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
    Ok(())
}
