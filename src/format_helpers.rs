use terminal_size::{Width, terminal_size};
use colored::Colorize;

pub fn centered_header(header_text: String) -> String {

    let term_width: u16 = get_terminal_width();

    format!("{text:#^width$}", text = header_text.red(), width = term_width as usize)
    
}

fn get_terminal_width() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        w
    } else {
        80
    }
}
