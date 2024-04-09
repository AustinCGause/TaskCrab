use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a task
    Add(AddArgs),
    /// View tasks
    View(ViewArgs),
    /// Mark a task as complete
    Complete,
    /// Delete a task
    Delete(DeleteArgs),

    // ################################################################################
    /// TESTING ONLY
    Clear,
    // ################################################################################
}

#[derive(Args)]
pub struct AddArgs {
    /// Task to add
    pub desc: Vec<String>,

    /// Due date of task
    #[arg(short, long, help = "Set an optional due date in MDY Format")]
    pub due: Option<String>,
}

#[derive(Args)]
pub struct ViewArgs {
    /// Specify the view type
    #[command(subcommand)]
    pub view_type: ViewType,
}

#[derive(Subcommand)]
pub enum ViewType {
    /// View in progress and completed tasks
    All,
    /// View only in progress tasks
    InProgress,
    /// View only completed tasks
    Completed,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Numerical index of task to delete
    pub index: u32,
}
