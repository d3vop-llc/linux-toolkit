use crate::utils::{ansi, colors, commands, console};
use crate::variables::colors as color_variable;
use crate::{pages, pages::defaults};
use std::io;

pub fn generate_page() {
    console::clear_console();

    let spacing: &str = "                                         ";
    // let seperator: String = format!("\x1b[45m                                         \x1b[0m");

    defaults::ui::title_bar(Some("    Antivirus"));

    println!(
        "{}",
        ansi::style(
            spacing,
            &format!(
                "{};{}",
                color_variable::TEXT_COLOR_PAGE,
                color_variable::BG_COLOR_PAGE
            )
        )
    );
    println!(
        "{}",
        ansi::style(
            " 1. Check for Rootkits                   ",
            &format!(
                "{};{}",
                color_variable::TEXT_COLOR_PAGE,
                color_variable::BG_COLOR_PAGE
            )
        )
    );
    println!(
        "{}",
        ansi::style(
            " 0. Return                               ",
            &format!(
                "{};{}",
                color_variable::TEXT_COLOR_PAGE,
                color_variable::BG_COLOR_PAGE
            )
        )
    );
    println!(
        "{}",
        ansi::style(
            spacing,
            &format!(
                "{};{}",
                color_variable::TEXT_COLOR_PAGE,
                color_variable::BG_COLOR_PAGE
            )
        )
    );

    defaults::ui::please_enter_choice();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect(&colors::output_background_color(
            "red",
            "Failed to read line",
        ));
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
            let args: Vec<&str> = vec![
                "sh",
                "-c",
                "apt update && apt install chkrootkit rkhunter -y",
            ];
            commands::run_command(
                true,
                "Installing chkrootkit and rkhunter...",
                "Failed to install chkrootkit and rkhunter.",
                args,
                "Successfully installed chkrootkit and rkhunter.",
                "Failed to install chkrootkit and rkhunter.",
            );
            let args: Vec<&str> = vec!["sh", "-c", "chkrootkit"];
            commands::run_command(
                true,
                "CHKRootKit - Checking for rootkits...",
                "Failed to check for rootkits.",
                args,
                "Successfully scanned for rootkits.",
                "Failed to scan for rootkits.",
            );
            let args: Vec<&str> = vec!["sh", "-c", "rkhunter --check"];
            commands::run_command(
                true,
                "RKHunter - Checking for rootkits...",
                "Failed to check for rootkits.",
                args,
                "Successfully scanned for rootkits.",
                "Failed to scan for rootkits.",
            );
            generate_page();
        }
        0 => {
            print!("\x1B[2J\x1B[1;1H");
            println!("Returning...");
            pages::page_1::generate_page();
        }
        _ => {
            defaults::outputs::inval_choice("");
        }
    }
}
