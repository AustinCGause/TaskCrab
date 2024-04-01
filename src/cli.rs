use clap::{ Args, Parser, Subcommand };
// use sqlx::sqlite::SqlitePoolOptions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a task
    Add(AddArgs),
    /// View tasks
    View(ViewArgs),
    /// Delete a task
    Delete(DeleteArgs),
    /// Mark a task as complete
    Complete,
    /// TESTING ONLY
    Clear,
}

#[derive(Args)]
pub struct AddArgs {
    /// Task to add
    pub desc: Vec<String>,

    /// Due date of task
    #[arg(short, long, help="Set an optional due date in MDY Format")]
    pub due: Option<String>,
}

#[derive(Args)]
pub struct ViewArgs {
    all: Option<bool>,
    in_progress: Option<bool>,
    completed: Option<bool>,
}

#[derive(Args)]
pub struct DeleteArgs {
    pub index: u32,
}
