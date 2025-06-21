use std::env;

use clap::Parser;

#[path ="config.rs"]
mod config;

pub use config::Configuration;

#[derive(Parser)]
#[command(name = "hookify")]
#[command(version)]
pub enum Commands {
    #[command(about = "Show installed version", alias = "-v")]
    Version,

    #[command(about = "Initialize hookify config (.hookify.toml)")]
    Init,
}

/// # Version
///
/// Functions group for getting -V, -v or --version output.
fn format_version() -> String {
    format!("hookify {}", env!("CARGO_PKG_VERSION"))
}

pub fn version() -> Result<(), String> {
    println!("{}", format_version());
    Ok(())
}

pub fn initialize() -> Result<(), String> {
    let config: Configuration = Configuration::new();
    config.initialize_default().map_err(|e| format!("Can't create configuration file: {}", e))?;
    Ok(())
}

/// Unit tests for command mod.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_version() {
        // Check name of util output
        assert!(format_version().starts_with("hookify"));

        // Check version output
        assert!(format_version().contains(env!("CARGO_PKG_VERSION")));
    }
}
