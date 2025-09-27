use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::Local;

use crate::error::{AppError, AppResult};

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
}

fn default_ssh_config_path() -> AppResult<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| AppError::ConfigError("无法获取用户主目录".to_string()))?;
    Ok(home.join(".ssh").join("config"))
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
