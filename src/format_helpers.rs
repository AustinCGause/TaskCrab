use std::collections::HashMap;

use terminal_size::{Width, terminal_size};
use colored::Colorize;
use tabled::{builder::Builder/* , settings::Style */};
use crate::task_manager::Task;

pub fn centered_header(header_text: String) -> String {

    let term_width: u16 = get_terminal_width();

    format!("{text:#^width$}", text = header_text.red(), width = term_width as usize)
    
}

pub fn output_tasks(tasks: &HashMap<u32, Task>) {
    let mut builder = Builder::default();

    for (index, task) in tasks.iter() {
        builder.push_record(vec![
            format!("{}", index),
            format!("{}", task.desc),
            format!("{}", task.due)
        ])       
    }
}

fn get_terminal_width() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        w
    } else {
        80
    }
}
