use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Network(reqwest::Error),
    Config(String),
    Api(String),
    Input(String),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {err}"),
            AppError::Network(err) => write!(f, "Network error: {err}"),
            AppError::Config(err) => write!(f, "Configuration error: {err}"),
            AppError::Api(err) => write!(f, "API error: {err}"),
            AppError::Input(err) => write!(f, "Input error: {err}"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err)
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(err: toml::ser::Error) -> Self {
        AppError::Config(format!("TOML serialization error: {err}"))
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        AppError::Config(format!("TOML deserialization error: {err}"))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Api(format!("JSON error: {err}"))
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
