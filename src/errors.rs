use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ConfigError {
    InitializationError(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ConfigError::InitializationError(msg) => write!(f, "Initialization failed: {}", msg),
        }
    }
}

impl Error for ConfigError {}
