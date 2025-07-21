use crate::utils::ansi;
use crate::variables::colors as color_variable;

pub fn title_bar(title_text: Option<&str>) {
    let txt_d = ansi::style(
        "D",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );
    let txt_3 = ansi::style(
        "3",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );
    let txt_vop = ansi::style(
        "vop",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );

    let title_text_background_color = color_variable::BG_COLOR_GREEN;
    let title_text_color = color_variable::TEXT_COLOR_BLACK;

    let title_text_title = ansi::style(
        title_text.unwrap_or("Linux Toolkit"),
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );
    let title_text_title_seperator = ansi::style(
        " - ",
        &format!(
            "{};1;{}",
            color_variable::TEXT_COLOR_GREY,
            color_variable::BG_COLOR_GREEN
        ),
    );

    let title_text_output = format!(
        "\x1b[{};{}m          {}{}\x1b[0m{}{}{}\x1b[{};{}m          \x1b[0m",
        title_text_background_color,
        title_text_color,
        title_text_title,
        title_text_title_seperator,
        txt_d,
        txt_3,
        txt_vop,
        title_text_background_color,
        title_text_color
    );

    println!("{}", title_text_output);
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
