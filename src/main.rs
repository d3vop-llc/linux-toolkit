use std::{io, process::Command, thread, time::Duration};

fn page_1() {
    print!("\x1B[2J\x1B[1;1H");
    println!("1. Update System Packages");
    println!("0. Exit");
    println!("Please enter your choice:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("\x1b[31mFailed to read line\x1b[0m");
    let choice: u8 = input.trim().parse().expect("Please enter a valid number");
    match choice {
        1 => {
            let args: Vec<&str> = vec!["sh", "-c", "apt-get update"];
            print!("\x1B[2J\x1B[1;1H");
            println!("\x1b[42;1mUpdating system packages...\x1b[0m");
            let status = Command::new("sudo")
                .args(&args)
                .status()
                .expect("\x1b[41;1mFailed to execute update command\x1b[0m");
            if status.success() {
                println!("\x1b[42;1mSystem packages updated successfully.\x1b[0m");
            } else {
                println!("\x1b[41;1mFailed to update system packages.\x1b[0m");
            }
            thread::sleep(Duration::from_secs(2));
            page_1();
        }
        0 => {
            print!("\x1B[2J\x1B[1;1H");
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            print!("\x1B[2J\x1B[1;1H");
            println!("\x1b[41;1mInvalid choice. Please try again.\x1b[0m");
            page_1();
        }
    }
}

fn main() {
    println!("Linux Toolkit - D3vop");

    println!("Page #1");

    page_1();
}
