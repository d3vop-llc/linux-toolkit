use crate::utils::{ansi, colors, commands, console};
use crate::variables::colors as color_variable;
use crate::{pages, pages::defaults};
use std::io;

pub fn generate_page() {
    console::clear_console();

    let spacing: &str = "                                         ";
    // let seperator: String = format!("\x1b[45m                                         \x1b[0m");

    defaults::ui::title_bar(Some("Linux Toolkit"));

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
            " 1. Update System Packages               ",
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
            " 2. Antivirus                            ",
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
            " 0. Exit                                 ",
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
            let args: Vec<&str> = vec!["sh", "-c", "apt-get update"];
            commands::run_command(
                true,
                "Updating system packages...",
                "Failed to execute update command",
                args,
                "System packages updated successfully.",
                "Failed to update system packages.",
            );
            generate_page();
        }
        2 => {
            pages::page_2::generate_page();
        }
        3 => {
            pages::security::generate_page();
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
