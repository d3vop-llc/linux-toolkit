use anyhow::Result;
use clap::{Arg, Command};
use std::io;

mod app;
mod commands;
mod config;
mod ui;
mod utils;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("linux-toolkit")
        .version("0.0.14")
        .author("D3vOp LLC")
        .about("A comprehensive toolkit for Linux system administration and cybersecurity")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .arg(
            Arg::new("command")
                .short('x')
                .long("execute")
                .value_name("COMMAND")
                .help("Execute a specific command directly"),
        )
        .get_matches();

    // Initialize configuration
    let config_path = matches.get_one::<String>("config");
    let config = config::Config::load(config_path)?;

    // Check if we should execute a command directly
    if let Some(command) = matches.get_one::<String>("command") {
        return commands::execute_direct_command(command, &config).await;
    }

    // Start the interactive TUI
    let mut terminal = ui::setup_terminal()?;
    let mut app = App::new(config);
    let result = app.run(&mut terminal).await;
    ui::restore_terminal(&mut terminal)?;

    result
}
