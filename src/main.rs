mod app_config;
mod cli;
mod task_manager;

use crate::{app_config::AppConfig, cli::{Cli, Commands}};
use clap::Parser;
// use sqlx::sqlite::SqlitePoolOptions;

fn main() {

    let config = AppConfig::new();

    if let Err(e) = config.ensure_setup() {
        eprintln!("Error during setup: {}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Add(_add_args) => todo!(),
        Commands::View(_view_args) => todo!(),
        Commands::Delete => todo!(),
        Commands::Complete => todo!(),
    };

}
