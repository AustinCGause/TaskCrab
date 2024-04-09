// use terminal_size::{Width, terminal_size};
use tabled::{builder::Builder, settings::Style};
use crate::{task_manager::Task, cli::ViewType};

// Keeping this just in case I decide to use it later
// pub fn output_centered_header(header_text: String) {
//
//     let term_width: u16 = get_terminal_width();
//
//     println!("\n{text:#^width$}\n", text = header_text.red(), width = term_width as usize)
//
// }

// TODO: Implement seperate viewing types with proper formatting
// Might be more semantic to keep the logic in the task_manager method and move the actual
// formatting parts to this file.
pub fn output_tasks(tasks: &Vec<Task>, view_type: ViewType) {

    let mut builder = Builder::default();

    builder.insert_record(0, vec![
        "Task Description",
        "Due Date"
    ]);

    match view_type {
        ViewType::All => {
            for task in tasks {
                builder.push_record(vec![
                    format!("{}", task.desc),
                    format!("{}", task.due)
                ])
            }
        },
        ViewType::InProgress => {
            todo!()
        }
        ViewType::Completed => {
            todo!()
        }
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
