use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// Include the embedded scripts
include!(concat!(env!("OUT_DIR"), "/embedded_scripts.rs"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptCommand {
    pub name: String,
    pub description: String,
    pub script: String,
    pub usage: String,
    pub requires_sudo: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptCategory {
    pub name: String,
    pub description: String,
    pub directory: String,
    pub commands: Vec<ScriptCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptsConfig {
    pub scripts: HashMap<String, ScriptCategory>,
}

pub struct ScriptManager {
    pub config: ScriptsConfig,
    pub scripts_dir: PathBuf,
}

impl ScriptManager {
    pub fn new(base_path: &Path) -> Result<Self> {
        let scripts_dir = base_path.join("scripts");

        // Extract embedded scripts if they don't exist
        Self::extract_embedded_scripts(&scripts_dir)?;

        let config_path = scripts_dir.join("scripts.toml");

        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            toml::from_str(&content)?
        } else {
            // Create default config if it doesn't exist
            let default_config = Self::create_default_config();
            let content = toml::to_string_pretty(&default_config)?;
            fs::write(&config_path, content)?;
            default_config
        };

        // Create directories for all categories in the config
        for (_, category) in &config.scripts {
            let category_dir = scripts_dir.join(&category.directory);
            fs::create_dir_all(&category_dir)?;

            // Create default script files if they don't exist
            for command in &category.commands {
                let script_path = category_dir.join(&command.script);
                if !script_path.exists() {
                    let default_script_content =
                        Self::create_default_script(&command.script, &command.description);
                    fs::write(&script_path, default_script_content)?;

                    // Make script executable on Unix systems
                    #[cfg(unix)]
                    {
                        if script_path.extension().and_then(|s| s.to_str()) == Some("sh") {
                            use std::os::unix::fs::PermissionsExt;
                            let mut perms = fs::metadata(&script_path)?.permissions();
                            perms.set_mode(0o755); // rwxr-xr-x
                            fs::set_permissions(&script_path, perms)?;
                        }
                    }

                    // Created default script (silent for TUI)
                }
            }
        }

        Ok(Self {
            config,
            scripts_dir,
        })
    }

    /// Extract embedded scripts to the file system if they don't exist
    fn extract_embedded_scripts(scripts_dir: &Path) -> Result<()> {
        // Ensure the scripts directory exists
        fs::create_dir_all(scripts_dir)?;

        let embedded_scripts = get_embedded_scripts();

        for (relative_path, content) in embedded_scripts {
            let target_path = scripts_dir.join(relative_path);

            // Only extract if the file doesn't exist or if we're updating
            if !target_path.exists() {
                // Create parent directories
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Write the embedded content
                fs::write(&target_path, content)?;

                // Make shell scripts executable on Unix systems
                #[cfg(unix)]
                {
                    if target_path.extension().and_then(|s| s.to_str()) == Some("sh") {
                        use std::os::unix::fs::PermissionsExt;
                        let mut perms = fs::metadata(&target_path)?.permissions();
                        perms.set_mode(0o755); // rwxr-xr-x
                        fs::set_permissions(&target_path, perms)?;
                    }
                }

                // Extracted script (silent for TUI)
            }
        }

        Ok(())
    }

    /// Get the executable directory (where the binary is located)
    pub fn get_executable_dir() -> Result<PathBuf> {
        let exe_path = std::env::current_exe()?;
        Ok(exe_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf())
    }

    /// Create a new ScriptManager using the directory where the executable is located
    /// Falls back to user data directory if executable directory is not writable
    pub fn new_from_exe() -> Result<Self> {
        let exe_dir = Self::get_executable_dir()?;

        // Test if we can write to the executable directory
        if Self::is_directory_writable(&exe_dir) {
            Self::new(&exe_dir)
        } else {
            // If that fails (e.g., no write permissions), use user data directory
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
            let user_scripts_dir = home_dir.join(".local").join("share").join("linux-toolkit");
            Self::new(&user_scripts_dir)
        }
    }

    /// Check if a directory is writable by trying to create a test file
    fn is_directory_writable(dir: &Path) -> bool {
        // If directory doesn't exist, try to create it first
        if !dir.exists() {
            if let Err(_) = fs::create_dir_all(dir) {
                return false;
            }
        }

        // Try to create a temporary file to test write permissions
        let test_file = dir.join(".write_test");
        match fs::write(&test_file, b"test") {
            Ok(_) => {
                // Clean up the test file
                let _ = fs::remove_file(&test_file);
                true
            }
            Err(_) => false,
        }
    }

    fn create_default_config() -> ScriptsConfig {
        let mut scripts = HashMap::new();

        // Create a default network category with example scripts
        let network_category = ScriptCategory {
            name: "Network Security".to_string(),
            description: "Network analysis and security tools".to_string(),
            directory: "network".to_string(),
            commands: vec![ScriptCommand {
                name: "Port Scanner".to_string(),
                description: "See what ports are currently active on the system.".to_string(),
                script: "active_ports.sh".to_string(),
                usage: "active_ports.sh".to_string(),
                requires_sudo: true,
                tags: vec![
                    "network".to_string(),
                    "security".to_string(),
                    "ports".to_string(),
                    "active".to_string(),
                ],
            }],
        };

        scripts.insert("network".to_string(), network_category);

        ScriptsConfig { scripts }
    }

    fn create_default_script(script_name: &str, description: &str) -> String {
        match script_name {
            "active_ports.sh" => r#"#!/bin/bash
# Linux Toolkit Script: Active Ports Scanner
# Description: See what ports are currently active on the system.

echo "=== Active Network Connections ==="
echo

# Check if netstat is available
if command -v netstat >/dev/null 2>&1; then
    echo "Using netstat to show active connections:"
    netstat -tuln
elif command -v ss >/dev/null 2>&1; then
    echo "Using ss to show active connections:"
    ss -tuln
else
    echo "Neither netstat nor ss are available. Please install net-tools or iproute2."
    exit 1
fi

echo
echo "=== Listening Ports Summary ==="
if command -v netstat >/dev/null 2>&1; then
    netstat -tuln | grep LISTEN | awk '{print $4}' | sort -u
elif command -v ss >/dev/null 2>&1; then
    ss -tuln | grep LISTEN | awk '{print $4}' | sort -u
fi
"#
            .to_string(),
            _ => {
                format!(
                    r#"#!/bin/bash
# Linux Toolkit Script: {}
# Description: {}

echo "This is a placeholder script for: {}"
echo "Description: {}"
echo "Please edit this script to add your functionality."

# Add your script logic here
echo "Script executed successfully!"
"#,
                    script_name, description, script_name, description
                )
            }
        }
    }

    pub fn list_available_scripts(&self) -> Vec<(String, Vec<ScriptCommand>)> {
        let mut scripts = Vec::new();

        for (_, category) in &self.config.scripts {
            let mut available_commands = Vec::new();

            for command in &category.commands {
                let script_path = self
                    .scripts_dir
                    .join(&category.directory)
                    .join(&command.script);

                if script_path.exists() {
                    available_commands.push(command.clone());
                }
            }

            if !available_commands.is_empty() {
                scripts.push((category.name.clone(), available_commands));
            }
        }

        scripts
    }

    pub async fn execute_script(
        &self,
        script_path: &Path,
        args: &[String],
        use_sudo: bool,
    ) -> Result<String> {
        use std::process::Stdio;
        use tokio::process::Command;

        let mut cmd = if use_sudo && !crate::utils::is_root() {
            let mut sudo_cmd = Command::new("sudo");
            if cfg!(target_os = "windows") {
                // On Windows, we might use PowerShell with elevated privileges
                sudo_cmd = Command::new("powershell");
                sudo_cmd.arg("-Command");
                sudo_cmd.arg(format!("& '{}' {}", script_path.display(), args.join(" ")));
            } else {
                sudo_cmd.arg(script_path);
                for arg in args {
                    sudo_cmd.arg(arg);
                }
            }
            sudo_cmd
        } else {
            let mut script_cmd = if cfg!(target_os = "windows") {
                // On Windows, execute bash scripts through Git Bash or WSL if available
                if which::which("bash").is_ok() {
                    let mut bash_cmd = Command::new("bash");
                    bash_cmd.arg(script_path);
                    bash_cmd
                } else {
                    // Fallback to PowerShell
                    let mut ps_cmd = Command::new("powershell");
                    ps_cmd.arg("-File");
                    ps_cmd.arg(script_path);
                    ps_cmd
                }
            } else {
                Command::new("bash")
            };

            if !cfg!(target_os = "windows") || which::which("bash").is_ok() {
                script_cmd.arg(script_path);
            }

            for arg in args {
                script_cmd.arg(arg);
            }
            script_cmd
        };

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = cmd.output().await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            let error_msg = if !stderr.is_empty() {
                format!("Script execution failed: {}", stderr)
            } else {
                format!("Script failed with exit code: {}", output.status)
            };
            return Ok(error_msg);
        }

        if stdout.is_empty() && !stderr.is_empty() {
            Ok(stderr.to_string())
        } else {
            Ok(stdout.to_string())
        }
    }

    pub async fn execute_script_in_terminal(
        &self,
        script_path: &Path,
        args: &[String],
        use_sudo: bool,
    ) -> Result<()> {
        use std::process::Command as StdCommand;

        // Clear the terminal screen
        if cfg!(target_os = "windows") {
            let _ = StdCommand::new("cls").status();
        } else {
            let _ = StdCommand::new("clear").status();
        }

        println!("Executing script: {}", script_path.display());
        if !args.is_empty() {
            println!("Arguments: {}", args.join(" "));
        }
        if use_sudo {
            println!("⚠️  This script requires elevated privileges");
        }
        println!("{}", "=".repeat(60));
        println!();

        let mut cmd = if use_sudo && !crate::utils::is_root() {
            if cfg!(target_os = "windows") {
                // On Windows, try to run with elevated privileges
                let mut ps_cmd = StdCommand::new("powershell");
                ps_cmd.arg("-Command");
                ps_cmd.arg(format!(
                    "Start-Process -FilePath 'bash' -ArgumentList '{}' -Verb RunAs -Wait",
                    script_path.display()
                ));
                ps_cmd
            } else {
                let mut sudo_cmd = StdCommand::new("sudo");
                sudo_cmd.arg("bash");
                sudo_cmd.arg(script_path);
                for arg in args {
                    sudo_cmd.arg(arg);
                }
                sudo_cmd
            }
        } else {
            let mut script_cmd = if cfg!(target_os = "windows") {
                if which::which("bash").is_ok() {
                    let mut bash_cmd = StdCommand::new("bash");
                    bash_cmd.arg(script_path);
                    bash_cmd
                } else {
                    let mut ps_cmd = StdCommand::new("powershell");
                    ps_cmd.arg("-File");
                    ps_cmd.arg(script_path);
                    ps_cmd
                }
            } else {
                let mut bash_cmd = StdCommand::new("bash");
                bash_cmd.arg(script_path);
                bash_cmd
            };

            for arg in args {
                script_cmd.arg(arg);
            }
            script_cmd
        };

        match cmd.status() {
            Ok(status) => {
                println!();
                if status.success() {
                    println!("✅ Script completed successfully");
                } else {
                    println!("❌ Script failed with exit code: {}", status);
                }
            }
            Err(e) => {
                println!("❌ Failed to execute script: {}", e);
            }
        }

        println!("\nPress Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        Ok(())
    }
}
