use crate::pages::defaults;
use crate::utils::{ colors, commands, console };
use std::io;

pub fn generate_page() {
    console::clear_console();
    println!("1. Update System Packages");
    println!("0. Exit");
    println!("Please enter your choice:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect(&colors::output_background_color("red", "Failed to read line"));
    let choice: u8 = match input.trim().parse::<u8>() {
        // returns Result<T, std::num::ParseIntError>
        Ok(num) => num,
        Err(_) => {
            defaults::outputs::inval_choice("Invalid input, please enter a number.");
            return generate_page();
        }
    };
    match choice {
        1 => {
            let args: Vec<&str> = vec!["sh", "-c", "apt-get update"];
            commands::run_command(
                true,
                "Updating system packages...",
                "Failed to execute update command",
                args,
                "System packages updated successfully.",
                "Failed to update system packages."
            );
            generate_page();
        }
        0 => {
            print!("\x1B[2J\x1B[1;1H");
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            defaults::outputs::inval_choice("");
        }
    }
}
