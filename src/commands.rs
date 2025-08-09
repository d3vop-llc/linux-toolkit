use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command as TokioCommand;

use crate::config::Config;
use crate::scripts::ScriptManager;

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
    let mut categories = load_builtin_categories();

    // Try to load and merge script-based commands
    match ScriptManager::new_from_exe() {
        Ok(script_manager) => {
            merge_script_commands(&mut categories, &script_manager);
        }
        Err(e) => {
            eprintln!("Warning: Failed to initialize script manager: {}", e);
            eprintln!("Scripts folder may not be available");
        }
    }

    // Sort categories alphabetically by name
    categories.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort commands within each category alphabetically by name
    for category in &mut categories {
        category.commands.sort_by(|a, b| a.name.cmp(&b.name));
    }

    categories
}

fn load_builtin_categories() -> Vec<CommandCategory> {
    vec![CommandCategory {
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
                description: "Find SUID/SGID files (potential privilege escalation)".to_string(),
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
    }]
}

pub async fn execute_command_in_terminal(command: &Command, config: &Config) -> Result<()> {
    use std::process::Command as StdCommand;

    // Handle script commands specially
    if command.command == "script" {
        return execute_script_command_in_terminal(command).await;
    }

    // Clear the terminal screen before executing the command
    if cfg!(target_os = "windows") {
        let _ = StdCommand::new("cls").status();
    } else {
        let _ = StdCommand::new("clear").status();
    }

    // Build the command with proper shell handling
    let has_shell_operators = command.args.iter().any(|arg| {
        arg.contains("&&") || arg.contains("|") || arg.contains(">") || arg.contains("<")
    });

    let should_use_sudo = command.requires_sudo;

    let (final_command, final_args) = if has_shell_operators {
        let full_command = format!("{} {}", command.command, command.args.join(" "));
        if should_use_sudo && !crate::utils::is_root() {
            (
                "sudo".to_string(),
                vec!["sh".to_string(), "-c".to_string(), full_command],
            )
        } else {
            ("sh".to_string(), vec!["-c".to_string(), full_command])
        }
    } else {
        if should_use_sudo && !crate::utils::is_root() {
            let mut sudo_args = vec![command.command.clone()];
            sudo_args.extend(command.args.clone());
            ("sudo".to_string(), sudo_args)
        } else {
            (command.command.clone(), command.args.clone())
        }
    };

    // Show command info before execution
    println!("\n{}", "=".repeat(60));
    println!("Executing: {}", command.name);
    println!("Description: {}", command.description);
    if should_use_sudo {
        println!("⚠️  This command requires elevated privileges");
    }
    println!("Command: {}", command.usage);
    println!("{}", "=".repeat(60));
    println!();

    // Execute the command and capture the exit status
    let mut cmd = StdCommand::new(&final_command);
    cmd.args(&final_args);

    match cmd.status() {
        Ok(status) => {
            println!();
            if status.success() {
                println!("✅ Command completed successfully");
            } else {
                let exit_code = status.code().unwrap_or(-1);
                println!("❌ Command failed with exit code: {}", exit_code);

                // Check if it might be a permission issue and suggest retry
                if exit_code == 1 || exit_code == 126 || exit_code == 127 {
                    if !should_use_sudo && should_retry_with_sudo(command, config) {
                        println!("💡 This might be a permission issue. Retrying with sudo...");
                        println!();

                        // Retry with sudo
                        return execute_command_with_sudo_retry(command).await;
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to execute command: {}", e);

            // Check if command not found and suggest sudo retry
            if !should_use_sudo && should_retry_with_sudo(command, config) {
                println!("💡 Retrying with elevated privileges...");
                println!();

                return execute_command_with_sudo_retry(command).await;
            }
        }
    }

    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    Ok(())
}

async fn execute_script_command_in_terminal(command: &Command) -> Result<()> {
    // Get the script manager
    let script_manager = ScriptManager::new_from_exe()?;

    // The script name should be in the first argument
    if command.args.is_empty() {
        println!("❌ Error: No script specified");
        println!("\nPress Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        return Ok(());
    }

    let script_name = &command.args[0];

    // Find the script path by looking in all categories
    let mut script_path = None;
    for (_, category) in &script_manager.config.scripts {
        let potential_path = script_manager
            .scripts_dir
            .join(&category.directory)
            .join(script_name);

        if potential_path.exists() {
            script_path = Some(potential_path);
            break;
        }
    }

    let script_path = match script_path {
        Some(path) => path,
        None => {
            println!("❌ Error: Script '{}' not found", script_name);
            println!("\nPress Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            return Ok(());
        }
    };

    // Execute the script with any additional arguments
    let script_args: Vec<String> = command.args[1..].to_vec();

    match script_manager
        .execute_script_in_terminal(&script_path, &script_args, command.requires_sudo)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("❌ Script execution error: {}", e);
            println!("\nPress Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            Ok(())
        }
    }
}

async fn execute_command_with_sudo_retry(command: &Command) -> Result<()> {
    use std::process::Command as StdCommand;

    // Clear the terminal screen before retrying with sudo
    if cfg!(target_os = "windows") {
        let _ = StdCommand::new("cls").status();
    } else {
        let _ = StdCommand::new("clear").status();
    }

    let has_shell_operators = command.args.iter().any(|arg| {
        arg.contains("&&") || arg.contains("|") || arg.contains(">") || arg.contains("<")
    });

    let (final_command, final_args) = if has_shell_operators {
        let full_command = format!("{} {}", command.command, command.args.join(" "));
        (
            "sudo".to_string(),
            vec!["sh".to_string(), "-c".to_string(), full_command],
        )
    } else {
        let mut sudo_args = vec![command.command.clone()];
        sudo_args.extend(command.args.clone());
        ("sudo".to_string(), sudo_args)
    };

    println!("🔓 Executing with elevated privileges...");
    println!();

    let mut cmd = StdCommand::new(&final_command);
    cmd.args(&final_args);

    match cmd.status() {
        Ok(status) => {
            println!();
            if status.success() {
                println!("✅ Command completed successfully (elevated)");
            } else {
                println!(
                    "❌ Command failed even with elevated privileges (exit code: {})",
                    status.code().unwrap_or(-1)
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to execute command with sudo: {}", e);
        }
    }

    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    Ok(())
}

pub async fn execute_command(command: &Command, config: &Config) -> Result<String> {
    // First, try to execute the command normally
    let result = execute_command_internal(command, false).await;

    match &result {
        Ok(output) => {
            // Check if the output contains permission denied errors
            if is_permission_denied_error(output) && should_retry_with_sudo(command, config) {
                // Retry with sudo
                match execute_command_internal(command, true).await {
                    Ok(sudo_output) => Ok(format!("Command elevated with sudo:\n{}", sudo_output)),
                    Err(_) => result, // Return original result if sudo also fails
                }
            } else {
                result
            }
        }
        Err(_) => {
            // Check if this might be a permission issue and retry with sudo
            if should_retry_with_sudo(command, config) {
                match execute_command_internal(command, true).await {
                    Ok(sudo_output) => Ok(format!("Command elevated with sudo:\n{}", sudo_output)),
                    Err(_) => result, // Return original error if sudo also fails
                }
            } else {
                result
            }
        }
    }
}

async fn execute_command_internal(command: &Command, use_sudo: bool) -> Result<String> {
    // Handle script commands specially
    if command.command == "script" {
        return execute_script_command(command, use_sudo).await;
    }

    let mut cmd = if use_sudo && !crate::utils::is_root() {
        let mut sudo_cmd = TokioCommand::new("sudo");
        sudo_cmd.arg(&command.command);
        sudo_cmd
    } else {
        TokioCommand::new(&command.command)
    };

    // Add arguments
    let has_shell_operators = command.args.iter().any(|arg| {
        arg.contains("&&") || arg.contains("|") || arg.contains(">") || arg.contains("<")
    });

    if has_shell_operators {
        if use_sudo && !crate::utils::is_root() {
            cmd = TokioCommand::new("sudo");
            cmd.arg("sh");
            cmd.arg("-c");
            let full_command = format!("{} {}", command.command, command.args.join(" "));
            cmd.arg(full_command);
        } else {
            cmd = TokioCommand::new("sh");
            cmd.arg("-c");
            let full_command = format!("{} {}", command.command, command.args.join(" "));
            cmd.arg(full_command);
        }
    } else {
        for arg in &command.args {
            cmd.arg(arg);
        }
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let output = cmd.output().await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check for permission errors in stderr
    if !output.status.success() {
        let error_msg = if !stderr.is_empty() {
            format!("Error: {}", stderr)
        } else {
            format!("Command failed with exit code: {}", output.status)
        };

        // If it's a permission error, include that information
        if is_permission_denied_error(&stderr) {
            return Ok(format!(
                "Permission denied. Try running with elevated privileges.\n{}",
                error_msg
            ));
        }

        return Ok(error_msg);
    }

    if stdout.is_empty() && !stderr.is_empty() {
        Ok(stderr.to_string())
    } else {
        Ok(stdout.to_string())
    }
}

async fn execute_script_command(command: &Command, use_sudo: bool) -> Result<String> {
    // Get the script manager
    let script_manager = ScriptManager::new_from_exe()?;

    // The script name should be in the first argument
    if command.args.is_empty() {
        return Ok("Error: No script specified".to_string());
    }

    let script_name = &command.args[0];

    // Find the script path by looking in all categories
    let mut script_path = None;
    for (_, category) in &script_manager.config.scripts {
        let potential_path = script_manager
            .scripts_dir
            .join(&category.directory)
            .join(script_name);

        if potential_path.exists() {
            script_path = Some(potential_path);
            break;
        }
    }

    let script_path = match script_path {
        Some(path) => path,
        None => {
            return Ok(format!("Error: Script '{}' not found", script_name));
        }
    };

    // Execute the script with any additional arguments
    let script_args: Vec<String> = command.args[1..].to_vec();

    match script_manager
        .execute_script(&script_path, &script_args, use_sudo)
        .await
    {
        Ok(output) => Ok(output),
        Err(e) => Ok(format!("Script execution error: {}", e)),
    }
}

fn should_retry_with_sudo(command: &Command, config: &Config) -> bool {
    // Always retry with sudo if the command is marked as requiring sudo
    if command.requires_sudo {
        return true;
    }

    // If auto_sudo is enabled in config, retry for commands that might need elevation
    if config.behavior.auto_sudo {
        return true;
    }

    // For specific commands that commonly need sudo
    matches!(
        command.command.as_str(),
        "apt"
            | "yum"
            | "dnf"
            | "zypper"
            | "pacman"
            | "systemctl"
            | "service"
            | "mount"
            | "umount"
            | "iptables"
            | "ufw"
            | "firewall-cmd"
            | "netstat"
            | "tcpdump"
            | "nmap"
            | "iwlist"
            | "iwconfig"
    )
}

fn is_permission_denied_error(stderr: &str) -> bool {
    let permission_indicators = [
        "permission denied",
        "operation not permitted",
        "access denied",
        "insufficient privileges",
        "must be root",
        "sudo required",
        "you must be root",
        "run as root",
    ];

    let stderr_lower = stderr.to_lowercase();
    permission_indicators
        .iter()
        .any(|&indicator| stderr_lower.contains(indicator))
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

fn merge_script_commands(categories: &mut Vec<CommandCategory>, script_manager: &ScriptManager) {
    let script_categories = script_manager.list_available_scripts();

    for (script_category_name, mut script_commands) in script_categories {
        // Sort script commands alphabetically by name
        script_commands.sort_by(|a, b| a.name.cmp(&b.name));

        // Find existing category or create new one
        let existing_category = categories
            .iter_mut()
            .find(|cat| cat.name == script_category_name);

        if let Some(category) = existing_category {
            // Add script commands to existing category
            for script_cmd in script_commands {
                let command = Command {
                    name: format!("📜 {}", script_cmd.name),
                    description: script_cmd.description,
                    command: "script".to_string(), // Special marker for script commands
                    args: vec![script_cmd.script],
                    usage: script_cmd.usage,
                    tags: script_cmd.tags,
                    requires_sudo: script_cmd.requires_sudo,
                    category: script_category_name.clone(),
                };
                category.commands.push(command);
            }
        } else {
            // Create new category for scripts
            let mut commands = Vec::new();
            for script_cmd in script_commands {
                let command = Command {
                    name: format!("📜 {}", script_cmd.name),
                    description: script_cmd.description,
                    command: "script".to_string(), // Special marker for script commands
                    args: vec![script_cmd.script],
                    usage: script_cmd.usage,
                    tags: script_cmd.tags,
                    requires_sudo: script_cmd.requires_sudo,
                    category: script_category_name.clone(),
                };
                commands.push(command);
            }

            categories.push(CommandCategory {
                name: script_category_name,
                description: "Script-based commands".to_string(),
                commands,
            });
        }
    }
}
