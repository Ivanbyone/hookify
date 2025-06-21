use clap::Parser as _;

pub mod commands;

pub use commands::{Commands, version, initialize};

fn main() {
    let options = commands::Commands::parse();
    match options {
        Commands::Version => version().unwrap(),
        Commands::Init => if let Err(e) = initialize() {
            eprintln!("Ошибка инициализации: {}", e);
            std::process::exit(1); },
    }
}
