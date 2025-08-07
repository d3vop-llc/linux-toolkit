use std::process::Command;

/// Check if a command exists in the system PATH
pub fn command_exists(command: &str) -> bool {
    which::which(command).is_ok()
}

/// Check if the current user has sudo privileges
pub fn has_sudo() -> bool {
    Command::new("sudo")
        .args(["-n", "true"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get the current username
pub fn get_current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Check if running as root
pub fn is_root() -> bool {
    get_current_user() == "root" || std::env::var("EUID").unwrap_or_default() == "0"
}

/// Format bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if bytes < THRESHOLD {
        return format!("{} B", bytes);
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= (THRESHOLD as f64) && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Validate IP address
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}

/// Extract IP addresses from text
pub fn extract_ips(text: &str) -> Vec<String> {
    let ip_regex = regex::Regex
        ::new(
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"
        )
        .unwrap();

    ip_regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Check if a port is open
pub async fn is_port_open(host: &str, port: u16) -> bool {
    use tokio::net::TcpStream;
    use tokio::time::{timeout, Duration};

    let address = format!("{}:{}", host, port);
    timeout(Duration::from_secs(3), TcpStream::connect(&address))
        .await
        .is_ok()
}

/// Get system architecture
pub fn get_arch() -> String {
    std::env::consts::ARCH.to_string()
}

/// Get operating system
pub fn get_os() -> String {
    std::env::consts::OS.to_string()
}

/// Truncate string to specified length
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Escape shell arguments
pub fn escape_shell_arg(arg: &str) -> String {
    if arg
        .chars()
        .any(|c| " \t\n\r\"'\\$`()[]{}|&;<>?*".contains(c))
    {
        format!("'{}'", arg.replace('\'', "'\"'\"'"))
    } else {
        arg.to_string()
    }
}
