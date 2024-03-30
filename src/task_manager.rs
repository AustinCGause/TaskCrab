// use std::error::Error;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    desc: String,
    due: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn _new() -> Self {
        Tasks { tasks: Vec::new() }
    }

    pub fn _load_from_file(_file_path: &Path) {}

    pub fn _add_task(&mut self, _desc: String, _due: String) {}

    pub fn _delete_task(&mut self, _index:usize) {}
    
}
