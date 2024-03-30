use directories::ProjectDirs;
use std::{error::Error, fs::{self, File}, path::PathBuf};
use crate::task_manager:: Tasks;

pub struct AppConfig {
    file_path: PathBuf,
}

impl AppConfig {
    pub fn new() -> Self {
        let proj_dirs = ProjectDirs::from("org", "austincgause", "taskcrab")
            .expect("Could not find project directory");
        let data_local_dir = proj_dirs.data_local_dir();
        let file_path = data_local_dir.join("tasks.json");
        AppConfig { file_path }
    }

    pub fn ensure_setup(&self) -> Result<(), Box<dyn Error>> {

        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if !self.file_path.exists() {
            let tasks = Tasks { tasks: Vec::new() };
            let file = File::create(&self.file_path)?;
            serde_json::to_writer(file, &tasks)?;
        }

        Ok(())
    }
}
