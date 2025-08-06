use crate::pages;
use crate::utils::page_builder::PageBuilder;

pub fn generate_page() {
    PageBuilder::new("Linux Toolkit")
        .add_system_command(
            "1",
            "Update System Packages",
            "apt-get update",
            "Updating system packages",
        )
        .add_page("2", "Antivirus", pages::page_2::generate_page)
        .add_page("3", "Security", pages::security::generate_page)
        .add_page("4", "Network Tools", pages::network::generate_page)
        .add_separator()
        .add_exit("0")
        .display();
}
