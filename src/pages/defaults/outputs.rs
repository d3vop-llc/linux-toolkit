use crate::pages::page_1;
use std::{ thread, time::Duration };

pub fn invalChoice() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\x1b[41;1mInvalid choice. Please try again.\x1b[0m");
    thread::sleep(Duration::from_secs(1));
    page_1::generate_page();
}
