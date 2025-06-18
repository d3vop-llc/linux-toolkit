pub fn output_background_color(color: &str, text: &str) -> String {
    match color {
        "red" => {
            return format!("\x1b[41;1m{}\x1b[0m", text);
        }
        "green" => {
            return format!("\x1b[42;1m{}\x1b[0m", text);
        }
        "yellow" => {
            return format!("\x1b[43m{}\x1b[0m", text);
        }
        "blue" => {
            return format!("\x1b[44m{}\x1b[0m", text);
        }
        "magenta" => {
            return format!("\x1b[45m{}\x1b[0m", text);
        }
        "cyan" => {
            return format!("\x1b[46m{}\x1b[0m", text);
        }
        "white" => {
            return format!("\x1b[47m{}\x1b[0m", text);
        }
        _ => {
            return text.to_string();
        }
    }
}
