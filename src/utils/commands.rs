use std::{ process::Command, process::ExitStatus, thread, time::Duration };
use crate::utils::{ colors, console };

pub fn run_command(
    sudo: bool,
    message_1: &str,
    failure_message_1: &str,
    args: Vec<&str>,
    success_message_1: &str,
    failure_message_2: &str
) {
    console::clear_console();
    println!("{}", colors::output_background_color("magenta", message_1));
    let status: ExitStatus = if sudo {
        Command::new("sudo")
            .args(&args)
            .status()
            .expect(&format!("{}", colors::output_background_color("red", failure_message_1)))
    } else {
        Command::new(&args[0])
            .args(&args[1..])
            .status()
            .expect(&format!("{}", colors::output_background_color("red", failure_message_1)))
    };
    if status.success() {
        println!("{}", colors::output_background_color("green", success_message_1));
    } else {
        println!("{}", colors::output_background_color("red", failure_message_2));
    }
    thread::sleep(Duration::from_secs(2));
}
