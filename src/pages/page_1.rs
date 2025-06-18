use crate::pages::defaults;
use crate::utils;
use std::io;

pub fn generate_page() {
    print!("\x1B[2J\x1B[1;1H");
    println!("1. Update System Packages");
    println!("0. Exit");
    println!("Please enter your choice:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("\x1b[31mFailed to read line\x1b[0m");
    let choice: u8 = input.trim().parse().expect("Please enter a valid number");
    match choice {
        1 => {
            let args: Vec<&str> = vec!["sh", "-c", "apt-get update"];
            utils::commands::run_command(
                true,
                "Updating system packages...",
                "Failed to execute update command",
                args,
                "System packages updated successfully.",
                "Failed to update system packages.",
            );
            generate_page();
        }
        0 => {
            print!("\x1B[2J\x1B[1;1H");
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            defaults::outputs::invalChoice();
        }
    }
}
