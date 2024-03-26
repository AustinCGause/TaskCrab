// use sqlx::sqlite::SqlitePoolOptions;
// use std::env;
use clap::{ Args, Parser, Subcommand };

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
    /// Task to add
    task: Vec<String>,

    /// Due date of task
    #[arg(short, long, help="Set an optional due date in MDY Format")]
    due: Option<String>,
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add_task_params) => add_task(add_task_params.task, add_task_params.due),
        Commands::Delete => todo!(),
        Commands::Complete => todo!(),
    }

}

fn add_task(task: Vec<String>, due: Option<String>) {

    let formatted_task = task.join(" ");

    println!("{}", formatted_task);

    if let Some(date) = due {
        println!("{}", date);
    }

}
