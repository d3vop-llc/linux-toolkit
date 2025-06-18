use crate::pages::page_1;
use crate::utils::{colors, console};
use std::{thread, time::Duration};

pub fn invalChoice(message: &str) {
    console::clear_console();
    let message: String = if message.is_empty() {
        "Invalid choice, please try again.".to_string()
    } else {
        message.to_string()
    };
    println!("{}", colors::output_background_color("red", &message));
    thread::sleep(Duration::from_secs(1));
    page_1::generate_page();
}
