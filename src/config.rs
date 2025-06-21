#[path = "file_operations.rs"]
mod file_operations;

#[path = "errors.rs"]
mod errors;

pub use errors::ConfigError;
pub use file_operations::FileOperations;

const DEFAULT_DIRECTORY_PATH: &str = "./";
const DEFAULT_HOOKIFY_CONFIG_NAME: &str = ".hookify.toml";
const DEFAULT_HOOKIFY_CONFIG: &str = r#"[githooks]
pre-commit = ""
commit-msg = ""

[logging]
verbose = true
"#;

pub struct Configuration;

impl Configuration {
    pub fn new() -> Self {
        Configuration
    }

    fn initialize(
        &self,
        filename: &str,
        directory_path: &str,
        configuration: &str,
    ) -> std::io::Result<()> {
        let creator: FileOperations = FileOperations::new();
        creator.create_file(directory_path.to_string() + &filename, configuration)?;
        Ok(())
    }

    pub fn initialize_default(&self) -> Result<(), ConfigError> {
        self.initialize(
            DEFAULT_HOOKIFY_CONFIG_NAME,
            DEFAULT_DIRECTORY_PATH,
            DEFAULT_HOOKIFY_CONFIG,
        )
        .map_err(|e| ConfigError::InitializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {}
