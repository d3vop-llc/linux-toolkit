pub fn style(text: &str, color_code: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}
