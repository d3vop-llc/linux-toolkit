pub fn get_current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn is_root() -> bool {
    get_current_user() == "root" || std::env::var("EUID").unwrap_or_default() == "0"
}
