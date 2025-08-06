use crate::utils::{ansi, console};
use crate::variables::colors as color_variable;
use std::io;

/// Represents a menu option with display text and an action
#[derive(Clone)]
pub struct MenuOption {
    pub number: String,
    pub text: String,
    pub action: MenuAction,
}

/// Different types of actions a menu option can perform
#[derive(Clone)]
pub enum MenuAction {
    /// Navigate to another page/function
    Navigate(fn()),
    /// Execute a command
    Command {
        sudo: bool,
        args: Vec<String>,
        start_message: String,
        success_message: String,
        error_message: String,
    },
    /// Exit the application
    Exit,
    /// Go back to previous menu
    Back(fn()),
    /// Do nothing (for separators)
    None,
}

/// Represents a complete menu page
pub struct Menu {
    pub title: String,
    pub options: Vec<MenuOption>,
}

impl Menu {
    /// Create a new menu with the given title
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            options: Vec::new(),
        }
    }

    /// Add a navigation option to the menu
    pub fn add_navigation(&mut self, number: &str, text: &str, action: fn()) -> &mut Self {
        self.options.push(MenuOption {
            number: number.to_string(),
            text: text.to_string(),
            action: MenuAction::Navigate(action),
        });
        self
    }

    /// Add a command option to the menu
    pub fn add_command(
        &mut self,
        number: &str,
        text: &str,
        sudo: bool,
        args: Vec<&str>,
        start_message: &str,
        success_message: &str,
        error_message: &str,
    ) -> &mut Self {
        self.options.push(MenuOption {
            number: number.to_string(),
            text: text.to_string(),
            action: MenuAction::Command {
                sudo,
                args: args.iter().map(|s| s.to_string()).collect(),
                start_message: start_message.to_string(),
                success_message: success_message.to_string(),
                error_message: error_message.to_string(),
            },
        });
        self
    }

    /// Add a back/return option to the menu
    pub fn add_back(&mut self, number: &str, text: &str, action: fn()) -> &mut Self {
        self.options.push(MenuOption {
            number: number.to_string(),
            text: text.to_string(),
            action: MenuAction::Back(action),
        });
        self
    }

    /// Add an exit option to the menu
    pub fn add_exit(&mut self, number: &str, text: &str) -> &mut Self {
        self.options.push(MenuOption {
            number: number.to_string(),
            text: text.to_string(),
            action: MenuAction::Exit,
        });
        self
    }

    /// Add a separator line to the menu
    pub fn add_separator(&mut self) -> &mut Self {
        self.options.push(MenuOption {
            number: String::new(),
            text: String::new(),
            action: MenuAction::None,
        });
        self
    }

    /// Display the menu and handle user input
    pub fn display_and_handle(&self) {
        console::clear_console();
        self.display_title();
        println!();

        // Display options
        for option in &self.options {
            if option.number.is_empty() && option.text.is_empty() {
                println!(); // Separator
            } else {
                self.print_menu_option(&option.number, &option.text);
            }
        }

        println!();
        self.print_prompt();

        // Handle input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice = input.trim();
        self.handle_choice(choice);
    }

    /// Display the title bar
    fn display_title(&self) {
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

        let title_text_title = ansi::style(
            &self.title,
            &format!(
                "{};1;{}",
                color_variable::TEXT_COLOR_BLACK,
                color_variable::BG_COLOR_GREEN
            ),
        );
        let title_text_title_separator = ansi::style(
            " - ",
            &format!(
                "{};1;{}",
                color_variable::TEXT_COLOR_BLACK,
                color_variable::BG_COLOR_GREEN
            ),
        );

        let title_text_output = format!(
            "\x1b[{};{}m          {}{}\x1b[0m{}{}{}\x1b[{};{}m          \x1b[0m",
            color_variable::BG_COLOR_GREEN,
            color_variable::TEXT_COLOR_BLACK,
            title_text_title,
            title_text_title_separator,
            txt_d,
            txt_3,
            txt_vop,
            color_variable::BG_COLOR_GREEN,
            color_variable::TEXT_COLOR_BLACK
        );

        println!("{}", title_text_output);
    }

    /// Print a menu option
    fn print_menu_option(&self, number: &str, text: &str) {
        let option_number = ansi::style(number, color_variable::MENU_NUMBER_COLOR);
        let option_text = ansi::style(text, color_variable::MENU_OPTION_COLOR);
        println!("  {} {}", option_number, option_text);
    }

    /// Print the input prompt
    fn print_prompt(&self) {
        let prompt = ansi::style(
            "Enter your choice: ",
            &format!("{};1", color_variable::MENU_PROMPT_COLOR),
        );
        print!("{}", prompt);
    }

    /// Handle user choice
    fn handle_choice(&self, choice: &str) {
        // Find the matching option
        for option in &self.options {
            if option.number.trim_end_matches('.') == choice.trim() {
                match &option.action {
                    MenuAction::Navigate(func) => {
                        func();
                        return;
                    }
                    MenuAction::Command {
                        sudo,
                        args,
                        start_message,
                        success_message,
                        error_message,
                    } => {
                        self.execute_command(
                            *sudo,
                            args,
                            start_message,
                            success_message,
                            error_message,
                        );
                        return;
                    }
                    MenuAction::Exit => {
                        console::clear_console();
                        println!("Exiting...");
                        std::process::exit(0);
                    }
                    MenuAction::Back(func) => {
                        console::clear_console();
                        println!("Returning...");
                        func();
                        return;
                    }
                    MenuAction::None => {
                        // Should not happen for numbered options
                        continue;
                    }
                }
            }
        }

        // If no match found, show error and redisplay menu
        self.show_invalid_choice();
    }

    /// Execute a command
    fn execute_command(
        &self,
        sudo: bool,
        args: &[String],
        start_message: &str,
        success_message: &str,
        error_message: &str,
    ) {
        crate::utils::commands::run_command(
            sudo,
            start_message,
            error_message,
            args.iter().map(|s| s.as_str()).collect(),
            success_message,
            error_message,
        );
        // Return to this menu after command execution
        self.display_and_handle();
    }

    /// Show invalid choice message and redisplay menu
    fn show_invalid_choice(&self) {
        console::clear_console();
        let message = ansi::style(
            "Invalid choice, please try again.",
            color_variable::ERROR_COLOR,
        );
        println!("{}", message);
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.display_and_handle();
    }
}
