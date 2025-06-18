use std::{process::Command, process::ExitStatus, thread, time::Duration};

pub fn run_command(
    sudo: bool,
    message_1: &str,
    failure_message_1: &str,
    args: Vec<&str>,
    success_message_1: &str,
    failure_message_2: &str,
) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\x1b[42;1m{}\x1b[0m", message_1);
    let status: ExitStatus = if sudo {
        Command::new("sudo")
            .args(&args)
            .status()
            .expect(&format!("\x1b[41;1m{}\x1b[0m", failure_message_1))
    } else {
        Command::new(&args[0])
            .args(&args[1..])
            .status()
            .expect(&format!("\x1b[41;1m{}\x1b[0m", failure_message_1))
    };
    if status.success() {
        println!("\x1b[42;1m{}\x1b[0m", success_message_1);
    } else {
        println!("\x1b[41;1m{}\x1b[0m", failure_message_2);
    }
    thread::sleep(Duration::from_secs(2));
}
