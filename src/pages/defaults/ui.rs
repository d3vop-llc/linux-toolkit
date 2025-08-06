use crate::utils::ansi;
use crate::utils::console;
use crate::variables::colors as color_variable;

pub fn title_bar(title_text: Option<&str>) {
    let txt_d = ansi::style(
        "D",
        &format!("{};1;{}", color_variable::TEXT_COLOR_GREY, color_variable::BG_COLOR_GREEN)
    );
    let txt_3 = ansi::style(
        "3",
        &format!("{};1;{}", color_variable::TEXT_COLOR_GREY, color_variable::BG_COLOR_GREEN)
    );
    let txt_vop = ansi::style(
        "vop",
        &format!("{};1;{}", color_variable::TEXT_COLOR_GREY, color_variable::BG_COLOR_GREEN)
    );

    let title_text_title = ansi::style(
        title_text.unwrap_or("Linux Toolkit"),
        &format!("{};1;{}", color_variable::TEXT_COLOR_BLACK, color_variable::BG_COLOR_GREEN)
    );
    let title_text_title_seperator = ansi::style(
        " - ",
        &format!("{};1;{}", color_variable::TEXT_COLOR_BLACK, color_variable::BG_COLOR_GREEN)
    );

    let title_text_output = format!(
        "\x1b[{};{}m          {}{}\x1b[0m{}{}{}\x1b[{};{}m          \x1b[0m",
        color_variable::BG_COLOR_GREEN,
        color_variable::TEXT_COLOR_BLACK,
        title_text_title,
        title_text_title_seperator,
        txt_d,
        txt_3,
        txt_vop,
        color_variable::BG_COLOR_GREEN,
        color_variable::TEXT_COLOR_BLACK
    );

    println!("{}", title_text_output);
}

pub fn print_menu_option(number: &str, text: &str) {
    let option_number = ansi::style(number, color_variable::MENU_NUMBER_COLOR);
    let option_text = ansi::style(text, color_variable::MENU_OPTION_COLOR);
    println!("  {} {}", option_number, option_text);
}

pub fn print_menu(title: &str, options: &[(String, String)]) {
    console::clear_console();
    title_bar(Some(title));
    println!();

    for (number, text) in options {
        if number.is_empty() && text.is_empty() {
            println!(); // Separator
        } else {
            print_menu_option(number, text);
        }
    }

    println!();
    please_enter_choice();
}

pub fn please_enter_choice() {
    let prompt = ansi::style(
        "Enter your choice: ",
        &format!("{};1", color_variable::MENU_PROMPT_COLOR)
    );
    print!("{}", prompt);
}
