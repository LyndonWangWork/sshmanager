    pub fn reset_storage(&mut self) -> AppResult<()> {
        // 如果文件存在，则删除它
        if self.storage_path.exists() {
            std::fs::remove_file(&self.storage_path)?;
        }
        
        // 重新初始化存储
        let app_dir = self.storage_path.parent()
            .ok_or_else(|| AppError::ConfigError("无法获取存储目录".to_string()))?;
        
        // 确保目录存在（这会创建目录如果它不存在）
        std::fs::create_dir_all(app_dir)?;
        
        // 创建新的空存储文件
        std::fs::File::create(&self.storage_path)?;
        
        Ok(())
    }