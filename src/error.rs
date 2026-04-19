use thiserror::Error;

#[derive(Error, Debug)]
pub enum DisplayError {
    #[cfg(target_os = "windows")]
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),

    #[error("DDC/CI communication failed: {0}")]
    DdcCommunicationFailed(String),

    #[error("Feature not supported: {0}")]
    FeatureNotSupported(String),

    #[error("Monitor not found: {0}")]
    MonitorNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Platform not supported")]
    UnsupportedPlatform,
}
