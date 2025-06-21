use std::env;

use clap::Parser;

#[derive(Parser)]
#[command(name = "hookify")]
#[command(version)]
pub enum Commands {
    #[command(about = "Show installed version.", alias = "-v")]
    Version,
}

/// # Version
///
/// Functions group for getting -V, -v or --version output.
fn format_version() -> String {
    format!("hookify {}", env!("CARGO_PKG_VERSION"))
}

pub fn version() -> () {
    println!("{}", format_version());
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
