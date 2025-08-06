use crate::utils::menu::{ Menu, MenuAction };

/// A builder for easily creating standardized pages
pub struct PageBuilder {
    title: String,
    menu: Menu,
}

impl PageBuilder {
    /// Create a new page builder with the given title
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            menu: Menu::new(title),
        }
    }

    /// Add a navigation option (goes to another page)
    pub fn add_page(mut self, number: &str, text: &str, page_function: fn()) -> Self {
        self.menu.add_navigation(number, text, page_function);
        self
    }

    /// Add a simple system command
    pub fn add_system_command(
        mut self,
        number: &str,
        text: &str,
        command: &str,
        description: &str
    ) -> Self {
        let args = vec!["sh".to_string(), "-c".to_string(), command.to_string()];
        self.menu.add_command(
            number,
            text,
            true, // Most system commands need sudo
            args
                .iter()
                .map(|s| s.as_str())
                .collect(),
            &format!("{}...", description),
            &format!("{} completed successfully.", description),
            &format!("Failed to {}.", description.to_lowercase())
        );
        self
    }

    /// Add an apt install command
    pub fn add_apt_install(mut self, number: &str, text: &str, packages: &[&str]) -> Self {
        let package_list = packages.join(" ");
        let command = format!("apt update && apt install {} -y", package_list);
        self.add_system_command(number, text, &command, &format!("Installing {}", package_list))
    }

    /// Add a custom command with full control
    pub fn add_custom_command(
        mut self,
        number: &str,
        text: &str,
        sudo: bool,
        args: Vec<&str>,
        start_message: &str,
        success_message: &str,
        error_message: &str
    ) -> Self {
        self.menu.add_command(
            number,
            text,
            sudo,
            args,
            start_message,
            success_message,
            error_message
        );
        self
    }

    /// Add a separator line
    pub fn add_separator(mut self) -> Self {
        self.menu.add_separator();
        self
    }

    /// Add a back/return option
    pub fn add_back(mut self, number: &str, back_function: fn()) -> Self {
        self.menu.add_back(number, "Return to Main Menu", back_function);
        self
    }

    /// Add an exit option
    pub fn add_exit(mut self, number: &str) -> Self {
        self.menu.add_exit(number, "Exit");
        self
    }

    /// Build and display the page
    pub fn display(self) {
        self.menu.display_and_handle();
    }

    /// Get the menu for advanced customization
    pub fn get_menu(self) -> Menu {
        self.menu
    }
}

/// Macro to make creating simple pages even easier
#[macro_export]
macro_rules! create_page {
    ($title:expr, $($option:expr),* $(,)?) => {
        {
            let mut builder = crate::utils::page_builder::PageBuilder::new($title);
            $(
                builder = $option(builder);
            )*
            builder
        }
    };
}

/// Helper functions for common page patterns
pub fn page_option(
    number: &str,
    text: &str,
    page_func: fn()
) -> impl Fn(PageBuilder) -> PageBuilder {
    let number = number.to_string();
    let text = text.to_string();
    move |builder: PageBuilder| builder.add_page(&number, &text, page_func)
}

pub fn system_command(
    number: &str,
    text: &str,
    command: &str,
    description: &str
) -> impl Fn(PageBuilder) -> PageBuilder {
    let number = number.to_string();
    let text = text.to_string();
    let command = command.to_string();
    let description = description.to_string();
    move |builder: PageBuilder| builder.add_system_command(&number, &text, &command, &description)
}

pub fn apt_install(
    number: &str,
    text: &str,
    packages: &[&str]
) -> impl Fn(PageBuilder) -> PageBuilder {
    let number = number.to_string();
    let text = text.to_string();
    let packages: Vec<String> = packages
        .iter()
        .map(|s| s.to_string())
        .collect();
    move |builder: PageBuilder| {
        let package_refs: Vec<&str> = packages
            .iter()
            .map(|s| s.as_str())
            .collect();
        builder.add_apt_install(&number, &text, &package_refs)
    }
}

pub fn separator() -> impl Fn(PageBuilder) -> PageBuilder {
    |builder: PageBuilder| builder.add_separator()
}

pub fn back_option(number: &str, back_func: fn()) -> impl Fn(PageBuilder) -> PageBuilder {
    let number = number.to_string();
    move |builder: PageBuilder| builder.add_back(&number, back_func)
}

pub fn exit_option(number: &str) -> impl Fn(PageBuilder) -> PageBuilder {
    let number = number.to_string();
    move |builder: PageBuilder| builder.add_exit(&number)
}
