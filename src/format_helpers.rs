use std::collections::HashMap;

// use terminal_size::{Width, terminal_size};
use tabled::{builder::Builder, settings::Style};
use crate::task_manager::Task;

// pub fn output_centered_header(header_text: String) {
//
//     let term_width: u16 = get_terminal_width();
//
//     println!("\n{text:#^width$}\n", text = header_text.red(), width = term_width as usize)
//
// }

pub fn output_tasks(tasks: &HashMap<u32, Task>) {

    let mut builder = Builder::default();

    for (index, task) in tasks.iter() {
        builder.push_record(vec![
            format!("{}", index),
            format!("{}", task.desc),
            format!("{}", task.due)
        ])
    }

    builder.insert_record(0, vec![
        "Index",
        "Task Description",
        "Due Date"
    ]);

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{}", table);
}

// fn get_terminal_width() -> u16 {
//     if let Some((Width(w), _)) = terminal_size() {
//         w
//     } else {
//         80
//     }
// }
