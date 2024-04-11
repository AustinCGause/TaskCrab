// use terminal_size::{Width, terminal_size};
use crate::task_manager::Task;
use tabled::{builder::Builder, settings::Style};

// Keeping this just in case I decide to use it later
// pub fn output_centered_header(header_text: String) {
//
//     let term_width: u16 = get_terminal_width();
//
//     println!("\n{text:#^width$}\n", text = header_text.red(), width = term_width as usize)
//
// }

pub fn output_tasks(tasks: Vec<Task>) {
    let mut builder = Builder::default();

    builder.insert_record(0, vec!["Task Description", "Due Date", "Completed?"]);

    // TODO: Seperate the in-progress and completed tasks
    // Might be better to save this functionality for after tui integration

    for task in tasks {
        builder.push_record(vec![
            format!("{}", task.desc),
            format!("{}", task.due),
            format!("{}", task.complete),
        ])
    }

    let builder = builder.index();

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("{}", table);
}

// Keeping this just in case it turns out I need it
// fn get_terminal_width() -> u16 {
//     if let Some((Width(w), _)) = terminal_size() {
//         w
//     } else {
//         80
//     }
// }
