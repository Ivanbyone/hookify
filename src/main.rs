use clap::Parser as _;

mod commands;

pub use commands::{Commands, version};

fn main() {
    let options = commands::Commands::parse();
    match options {
        Commands::Version => version(),
    }
}
