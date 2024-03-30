// use std::error::Error;
use std::{error::Error, fs::File, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {

    pub fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    pub fn load_from_file(file_path: &Path) -> Result<Tasks, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let tasks = serde_json::from_reader(file)?;
        Ok(tasks)
    }

    pub fn add_task(&mut self, file: File, desc: String, due: String) -> Result<(), Box<dyn Error>> {
        self.tasks.push(Task::new(desc, due));
        serde_json::to_writer_pretty(file, self)?; // TODO: Change from _pretty in final build
        Ok(())
    }

    pub fn _delete_task(&mut self, _index:usize) {} // TODO: Implement delete task functionality

    pub fn clear_tasks(&mut self, file: File) -> Result<(), Box<dyn Error>> {
        self.tasks.clear();
        serde_json::to_writer(file, self)?;
        Ok(())
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    desc: String,
    due: String,
}

impl Task {

    pub fn new(desc: String, due: String) -> Self {
        Task {
            desc,
            due,
        }
    }

}
