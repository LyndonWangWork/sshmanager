use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("密钥生成错误: {0}")]
    KeyGenerationError(String),
    
    #[error("认证失败")]
    AuthenticationFailed,
    
    #[error("应用未初始化")]
    NotInitialized,
    
    #[error("无效的密钥类型")]
    InvalidKeyType,
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

pub type AppResult<T> = Result<T, AppError>;