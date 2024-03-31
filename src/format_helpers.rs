use terminal_size::{Width, terminal_size};

pub fn centered_header(header_text: &str, fill_char: char) -> String {

    let term_width: u16 = get_terminal_width();

    let padding_total = term_width.saturating_sub(header_text.len() as u16);
    let padding_side = padding_total / 2;
    let mut result = String::new();
    for _ in 0..padding_side {
        result.push(fill_char);
    }
    result.push_str(header_text);
    while result.len() < term_width as usize {
        result.push(fill_char);
    }
    result

}

fn get_terminal_width() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        w
    } else {
        80
    }
}
