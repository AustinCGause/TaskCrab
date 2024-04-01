use terminal_size::{Width, terminal_size};
use colored::Colorize;

pub fn centered_header(header_text: &str, _fill_char: char) -> String {

    let _term_width: u16 = get_terminal_width();

    format!("{:{}^{}{}}", _fill_char, _term_width, header_text.red())
    
}

fn get_terminal_width() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        w
    } else {
        80
    }
}
