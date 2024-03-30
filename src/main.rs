use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
use clap::{ Args, Parser, Subcommand };
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
// use sqlx::sqlite::SqlitePoolOptions;

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

#[derive(Serialize, Deserialize)]
struct Task {
    desc: String,
    due: String,
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add_task_params) => {
            if let Err(e) = add_task(add_task_params.task, add_task_params.due) {
                eprintln!("Error adding task: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Delete => todo!(),
        Commands::Complete => todo!(),
    };

}

fn add_task(task: Vec<String>, due: Option<String>) -> Result<(), Box<dyn Error>> {

    if let Some(proj_dirs) = ProjectDirs::from("org", "austincgause", "taskcrab") {
        let data_local_dir = proj_dirs.data_local_dir();

        fs::create_dir_all(data_local_dir)?;

        let file_path: PathBuf = data_local_dir.join("tasks.json");
        
        let task = Task {
            desc: task.join(" "),
            due: due.unwrap_or_else(|| "none".to_string()),
        };

        let t = serde_json::to_string(&task)?;

        let mut file = File::create(file_path)?;

        file.write_all(t.as_bytes())?;
    } else {
        return Err("Could not find project directories".into());
    }

    Ok(())

}
