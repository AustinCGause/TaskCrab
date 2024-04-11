mod app_config;
mod cli;
mod format_helpers;
mod task_manager;

use crate::{
    app_config::AppConfig,
    cli::{Cli, Command},
};
use clap::Parser;
use std::error::Error;
use task_manager::Tasks;
// use sqlx::sqlite::SqlitePoolOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::new();

    config.ensure_setup()?;

    // Loads tasks from file with standard config path
    let mut tasks = Tasks::load_from_file(&config.file_path)?;

    let cli = Cli::parse();

    match cli.command {
        Command::Add(add_args) => tasks.add_task(
            config.get_file_for_write(false)?,
            add_args.desc.join(" "),
            add_args.due.unwrap_or_else(|| String::from("None")),
        )?,
        Command::View(view_args) => tasks.view_tasks(view_args.view_type)?,
        Command::Delete(delete_args) => {
            tasks.delete_task(config.get_file_for_write(true)?, delete_args.index)?
        }
        Command::Complete(complete_args) => {
            tasks.complete_task(config.get_file_for_write(true)?, complete_args.index)?
        }

        // ################################################################################
        // TEST METHOD - REMOVE IN FINAL BUILD
        Command::Clear => tasks.clear_tasks(config.get_file_for_write(true)?)?,
        // ################################################################################
    };

    Ok(())
}
