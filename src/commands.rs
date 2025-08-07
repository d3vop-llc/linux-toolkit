use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command as TokioCommand;

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub usage: String,
    pub tags: Vec<String>,
    pub requires_sudo: bool,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategory {
    pub name: String,
    pub description: String,
    pub commands: Vec<Command>,
}

pub fn load_categories() -> Vec<CommandCategory> {
    vec![
        // Network Security & Analysis
        CommandCategory {
            name: "Network Security".to_string(),
            description: "Network analysis and security tools".to_string(),
            commands: vec![
                Command {
                    name: "Network Scan".to_string(),
                    description: "Scan local network for active hosts".to_string(),
                    command: "nmap".to_string(),
                    args: vec!["-sn".to_string(), "192.168.1.0/24".to_string()],
                    usage: "nmap -sn 192.168.1.0/24".to_string(),
                    tags: vec![
                        "network".to_string(),
                        "scan".to_string(),
                        "reconnaissance".to_string(),
                    ],
                    requires_sudo: false,
                    category: "network".to_string(),
                },
                Command {
                    name: "Port Scan".to_string(),
                    description: "Scan for open ports on a target".to_string(),
                    command: "nmap".to_string(),
                    args: vec![
                        "-sS".to_string(),
                        "-O".to_string(),
                        "192.168.1.1".to_string(),
                    ],
                    usage: "nmap -sS -O <target>".to_string(),
                    tags: vec![
                        "network".to_string(),
                        "ports".to_string(),
                        "scan".to_string(),
                    ],
                    requires_sudo: true,
                    category: "network".to_string(),
                },
                Command {
                    name: "Network Connections".to_string(),
                    description: "Show active network connections".to_string(),
                    command: "netstat".to_string(),
                    args: vec!["-tuln".to_string()],
                    usage: "netstat -tuln".to_string(),
                    tags: vec![
                        "network".to_string(),
                        "connections".to_string(),
                        "monitoring".to_string(),
                    ],
                    requires_sudo: false,
                    category: "network".to_string(),
                },
                Command {
                    name: "WiFi Networks".to_string(),
                    description: "Scan for WiFi networks".to_string(),
                    command: "iwlist".to_string(),
                    args: vec!["scan".to_string()],
                    usage: "iwlist scan".to_string(),
                    tags: vec![
                        "wifi".to_string(),
                        "wireless".to_string(),
                        "scan".to_string(),
                    ],
                    requires_sudo: true,
                    category: "network".to_string(),
                },
            ],
        },
        // System Information & Monitoring
        CommandCategory {
            name: "System Info".to_string(),
            description: "System information and monitoring tools".to_string(),
            commands: vec![
                Command {
                    name: "System Overview".to_string(),
                    description: "Display comprehensive system information".to_string(),
                    command: "neofetch".to_string(),
                    args: vec![],
                    usage: "neofetch".to_string(),
                    tags: vec![
                        "system".to_string(),
                        "info".to_string(),
                        "overview".to_string(),
                    ],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
                Command {
                    name: "Process List".to_string(),
                    description: "Show running processes".to_string(),
                    command: "ps".to_string(),
                    args: vec!["aux".to_string()],
                    usage: "ps aux".to_string(),
                    tags: vec!["processes".to_string(), "monitoring".to_string()],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
                Command {
                    name: "Memory Usage".to_string(),
                    description: "Display memory usage information".to_string(),
                    command: "free".to_string(),
                    args: vec!["-h".to_string()],
                    usage: "free -h".to_string(),
                    tags: vec!["memory".to_string(), "ram".to_string(), "usage".to_string()],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
                Command {
                    name: "Disk Usage".to_string(),
                    description: "Show disk space usage".to_string(),
                    command: "df".to_string(),
                    args: vec!["-h".to_string()],
                    usage: "df -h".to_string(),
                    tags: vec![
                        "disk".to_string(),
                        "storage".to_string(),
                        "usage".to_string(),
                    ],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
                Command {
                    name: "CPU Info".to_string(),
                    description: "Display CPU information".to_string(),
                    command: "lscpu".to_string(),
                    args: vec![],
                    usage: "lscpu".to_string(),
                    tags: vec![
                        "cpu".to_string(),
                        "hardware".to_string(),
                        "info".to_string(),
                    ],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
            ],
        },
        // Security & Forensics
        CommandCategory {
            name: "Security".to_string(),
            description: "Security analysis and forensics tools".to_string(),
            commands: vec![
                Command {
                    name: "Check Failed Logins".to_string(),
                    description: "Display failed login attempts".to_string(),
                    command: "grep".to_string(),
                    args: vec!["Failed".to_string(), "/var/log/auth.log".to_string()],
                    usage: "grep Failed /var/log/auth.log".to_string(),
                    tags: vec![
                        "security".to_string(),
                        "logs".to_string(),
                        "authentication".to_string(),
                    ],
                    requires_sudo: true,
                    category: "security".to_string(),
                },
                Command {
                    name: "List Users".to_string(),
                    description: "Display all system users".to_string(),
                    command: "cat".to_string(),
                    args: vec!["/etc/passwd".to_string()],
                    usage: "cat /etc/passwd".to_string(),
                    tags: vec![
                        "users".to_string(),
                        "accounts".to_string(),
                        "system".to_string(),
                    ],
                    requires_sudo: false,
                    category: "security".to_string(),
                },
                Command {
                    name: "Check SUID Files".to_string(),
                    description: "Find SUID/SGID files (potential privilege escalation)"
                        .to_string(),
                    command: "find".to_string(),
                    args: vec![
                        "/".to_string(),
                        "-perm".to_string(),
                        "-4000".to_string(),
                        "-o".to_string(),
                        "-perm".to_string(),
                        "-2000".to_string(),
                        "2>/dev/null".to_string(),
                    ],
                    usage: "find / -perm -4000 -o -perm -2000 2>/dev/null".to_string(),
                    tags: vec![
                        "suid".to_string(),
                        "privilege".to_string(),
                        "escalation".to_string(),
                    ],
                    requires_sudo: true,
                    category: "security".to_string(),
                },
                Command {
                    name: "Open Files".to_string(),
                    description: "List open files and network connections".to_string(),
                    command: "lsof".to_string(),
                    args: vec!["-i".to_string()],
                    usage: "lsof -i".to_string(),
                    tags: vec![
                        "files".to_string(),
                        "network".to_string(),
                        "monitoring".to_string(),
                    ],
                    requires_sudo: true,
                    category: "security".to_string(),
                },
            ],
        },
        // Log Analysis
        CommandCategory {
            name: "Log Analysis".to_string(),
            description: "System log analysis and monitoring".to_string(),
            commands: vec![
                Command {
                    name: "System Logs".to_string(),
                    description: "View recent system log entries".to_string(),
                    command: "journalctl".to_string(),
                    args: vec!["-n".to_string(), "50".to_string()],
                    usage: "journalctl -n 50".to_string(),
                    tags: vec![
                        "logs".to_string(),
                        "system".to_string(),
                        "journal".to_string(),
                    ],
                    requires_sudo: false,
                    category: "logs".to_string(),
                },
                Command {
                    name: "Apache Access Log".to_string(),
                    description: "View Apache access log".to_string(),
                    command: "tail".to_string(),
                    args: vec![
                        "-n".to_string(),
                        "100".to_string(),
                        "/var/log/apache2/access.log".to_string(),
                    ],
                    usage: "tail -n 100 /var/log/apache2/access.log".to_string(),
                    tags: vec![
                        "apache".to_string(),
                        "web".to_string(),
                        "access".to_string(),
                    ],
                    requires_sudo: true,
                    category: "logs".to_string(),
                },
                Command {
                    name: "SSH Logs".to_string(),
                    description: "View SSH connection attempts".to_string(),
                    command: "grep".to_string(),
                    args: vec!["sshd".to_string(), "/var/log/auth.log".to_string()],
                    usage: "grep sshd /var/log/auth.log".to_string(),
                    tags: vec![
                        "ssh".to_string(),
                        "authentication".to_string(),
                        "logs".to_string(),
                    ],
                    requires_sudo: true,
                    category: "logs".to_string(),
                },
            ],
        },
        // File Operations
        CommandCategory {
            name: "File Operations".to_string(),
            description: "File and directory operations".to_string(),
            commands: vec![
                Command {
                    name: "Find Large Files".to_string(),
                    description: "Find files larger than 100MB".to_string(),
                    command: "find".to_string(),
                    args: vec![
                        "/".to_string(),
                        "-size".to_string(),
                        "+100M".to_string(),
                        "2>/dev/null".to_string(),
                    ],
                    usage: "find / -size +100M 2>/dev/null".to_string(),
                    tags: vec![
                        "files".to_string(),
                        "size".to_string(),
                        "cleanup".to_string(),
                    ],
                    requires_sudo: true,
                    category: "files".to_string(),
                },
                Command {
                    name: "Directory Sizes".to_string(),
                    description: "Show directory sizes in current location".to_string(),
                    command: "du".to_string(),
                    args: vec!["-sh".to_string(), "*".to_string()],
                    usage: "du -sh *".to_string(),
                    tags: vec![
                        "directories".to_string(),
                        "size".to_string(),
                        "disk".to_string(),
                    ],
                    requires_sudo: false,
                    category: "files".to_string(),
                },
                Command {
                    name: "Find Recent Files".to_string(),
                    description: "Find files modified in last 24 hours".to_string(),
                    command: "find".to_string(),
                    args: vec![
                        "/".to_string(),
                        "-mtime".to_string(),
                        "-1".to_string(),
                        "2>/dev/null".to_string(),
                    ],
                    usage: "find / -mtime -1 2>/dev/null".to_string(),
                    tags: vec![
                        "files".to_string(),
                        "recent".to_string(),
                        "modified".to_string(),
                    ],
                    requires_sudo: true,
                    category: "files".to_string(),
                },
            ],
        },
        // Quick Commands
        CommandCategory {
            name: "Quick Commands".to_string(),
            description: "Frequently used quick commands".to_string(),
            commands: vec![
                Command {
                    name: "Update System".to_string(),
                    description: "Update package lists and upgrade system".to_string(),
                    command: "apt".to_string(),
                    args: vec![
                        "update".to_string(),
                        "&&".to_string(),
                        "apt".to_string(),
                        "upgrade".to_string(),
                        "-y".to_string(),
                    ],
                    usage: "apt update && apt upgrade -y".to_string(),
                    tags: vec![
                        "update".to_string(),
                        "upgrade".to_string(),
                        "packages".to_string(),
                    ],
                    requires_sudo: true,
                    category: "system".to_string(),
                },
                Command {
                    name: "IP Address".to_string(),
                    description: "Show IP address information".to_string(),
                    command: "ip".to_string(),
                    args: vec!["addr".to_string(), "show".to_string()],
                    usage: "ip addr show".to_string(),
                    tags: vec![
                        "network".to_string(),
                        "ip".to_string(),
                        "interface".to_string(),
                    ],
                    requires_sudo: false,
                    category: "network".to_string(),
                },
                Command {
                    name: "Who Is Logged In".to_string(),
                    description: "Show who is currently logged in".to_string(),
                    command: "who".to_string(),
                    args: vec![],
                    usage: "who".to_string(),
                    tags: vec![
                        "users".to_string(),
                        "logged".to_string(),
                        "session".to_string(),
                    ],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
                Command {
                    name: "System Uptime".to_string(),
                    description: "Show system uptime and load".to_string(),
                    command: "uptime".to_string(),
                    args: vec![],
                    usage: "uptime".to_string(),
                    tags: vec![
                        "uptime".to_string(),
                        "load".to_string(),
                        "system".to_string(),
                    ],
                    requires_sudo: false,
                    category: "system".to_string(),
                },
            ],
        },
    ]
}

pub async fn execute_command(command: &Command, _config: &Config) -> Result<String> {
    let mut cmd = TokioCommand::new(&command.command);

    // Add arguments
    for arg in &command.args {
        // Handle shell operators like && and redirections
        if arg.contains("&&") || arg.contains("|") || arg.contains(">") {
            cmd = TokioCommand::new("sh");
            cmd.arg("-c");
            let full_command = format!("{} {}", command.command, command.args.join(" "));
            cmd.arg(full_command);
            break;
        } else {
            cmd.arg(arg);
        }
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let output = cmd.output().await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        if !stderr.is_empty() {
            return Ok(format!("Error: {}", stderr));
        } else {
            return Ok(format!("Command failed with exit code: {}", output.status));
        }
    }

    if stdout.is_empty() && !stderr.is_empty() {
        Ok(stderr.to_string())
    } else {
        Ok(stdout.to_string())
    }
}

pub async fn execute_direct_command(command_name: &str, config: &Config) -> Result<()> {
    let categories = load_categories();

    for category in &categories {
        for cmd in &category.commands {
            if cmd
                .name
                .to_lowercase()
                .contains(&command_name.to_lowercase())
            {
                println!("Executing: {}", cmd.name);
                let output = execute_command(cmd, config).await?;
                println!("{}", output);
                return Ok(());
            }
        }
    }

    println!("Command not found: {}", command_name);
    Ok(())
}
