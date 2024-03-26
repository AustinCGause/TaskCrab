mod add;

// use sqlx::sqlite::SqlitePoolOptions;
// use std::env;
use clap::{ Args, Parser, Subcommand };
use add::add_item;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand)]
enum Commands {
    /// Add a task
    Add(AddArgs),
    /// Delete a task
    Delete,
    /// Mark a task as complete
    Complete,
}

#[derive(Args)]
struct AddArgs {
    task: Vec<String>
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add_task_params) => add_item(add_task_params.task),
        Commands::Delete => todo!(),
        Commands::Complete => todo!(),
    }

}
