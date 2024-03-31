// use std::error::Error;
use std::{collections::HashMap, error::Error, fs::File, path::Path};
use serde::{Deserialize, Serialize};
use fastrand::i32;
use crate::format_helpers::centered_header;

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    // pub tasks: Vec<Task>,
    pub tasks: HashMap<i32, Task>,
}

impl Tasks {

    pub fn new() -> Self {
        Tasks { tasks: HashMap::new() }
    }

    pub fn load_from_file(file_path: &Path) -> Result<Tasks, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let tasks = serde_json::from_reader(file)?;
        Ok(tasks)
    }

    pub fn add_task(&mut self, file: File, desc: String, due: String) -> Result<(), Box<dyn Error>> {
        let index: i32 = self.tasks.len().try_into()?;
        self.tasks.insert(index, Task::new(desc, due));
        serde_json::to_writer_pretty(file, self)?; // TODO: Change from _pretty in final build
        Ok(())
    }

    pub fn view_tasks(&self) -> Result<(), Box<dyn Error>> {
        println!("{}", centered_header("TaskCrab", '#'));
        Ok(())
    }

    pub fn _delete_task(&mut self, _index:usize) {} // TODO: Implement delete task functionality

    pub fn clear_tasks(&mut self, file: File) -> Result<(), Box<dyn Error>> { // TODO: Remove this
        self.tasks.clear();
        serde_json::to_writer(file, self)?;
        Ok(())
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    desc: String,
    due: String,
    id: i32,
}

impl Task {

    pub fn new(desc: String, due: String) -> Self {
        Task {
            desc,
            due,
            id: generate_id(),
        }
    }

}

fn generate_id() -> i32 {
    i32(99..)
}
