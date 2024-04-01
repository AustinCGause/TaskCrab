mod app_config;
mod cli;
mod task_manager;
mod format_helpers;

use crate::{app_config::AppConfig, cli::{Cli, Commands}};
use std::error::Error;
use clap::Parser;
use task_manager::Tasks;
// use sqlx::sqlite::SqlitePoolOptions;

fn main() -> Result<(), Box<dyn Error>> {

    let config = AppConfig::new();

    config.ensure_setup()?;

    let mut tasks = Tasks::load_from_file(&config.file_path)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add_args) => {
            tasks.add_task(
                config.get_file_for_write(false)?,
                add_args.desc.join(" "),
                add_args.due.unwrap_or_else(|| String::from("No due date provided")),
            )?
        },
        Commands::View(_view_args) => tasks.view_tasks()?,
        Commands::Delete(delete_args) => {
            tasks.delete_task(
                config.get_file_for_write(true)?,
                delete_args.index,
            )?
        },
        Commands::Complete => todo!(),
        Commands::Clear => tasks.clear_tasks(config.get_file_for_write(true)?)?, // TODO: Remove this
    };

    Ok(())

}
