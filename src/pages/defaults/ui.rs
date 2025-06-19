use crate::utils::ansi;
use crate::variables::colors as color_variable;

pub fn title_bar() {
    let txt_d: String = ansi::style(
        "D",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );
    let txt_3: String = ansi::style(
        "3",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );
    let txt_vop: String = ansi::style(
        "vop",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );
    let title_text_background_color: &str = color_variable::BG_COLOR_GREEN;
    let title_text_color: &str = color_variable::TEXT_COLOR_BLACK;
    let title_text: String = format!(
        "\x1b[{};{}m          Linux Toolkit - \x1b[0m{}{}{}\x1b[{};{}m          \x1b[0m",
        title_text_background_color,
        title_text_color,
        txt_d,
        txt_3,
        txt_vop,
        title_text_background_color,
        title_text_color
    );
    println!("{}", title_text);
}

pub fn please_enter_choice() {
    println!(
        "{}",
        ansi::style(
            "Please enter your choice:                ",
            &format!(
                "{};{}",
                color_variable::TEXT_COLOR_BLACK,
                color_variable::BG_COLOR_GREEN
            )
        )
    );
}
