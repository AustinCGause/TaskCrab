use directories::ProjectDirs;
use std::{error::Error, fs::{self, File, OpenOptions}, path::PathBuf};
use crate::task_manager::Tasks;

pub struct AppConfig {
    pub file_path: PathBuf,
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
            let file = File::create(&self.file_path)?;
            let tasks = Tasks::new();
            serde_json::to_writer(file, &tasks)?;
        }

        Ok(())
    }

    pub fn get_file_for_write(&self, truncate: bool) -> Result<File, Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(truncate)
            .open(&self.file_path)?;
        Ok(file)
    }

    pub fn _get_file_for_read(&self) -> Result<File, Box<dyn Error>> {
        let file = File::open(&self.file_path)?;
        Ok(file)
    }
}
