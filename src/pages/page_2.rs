use crate::utils::page_builder::PageBuilder;
use crate::pages;

pub fn generate_page() {
    PageBuilder::new("Antivirus")
        .add_apt_install("1", "Check for Rootkits", &["chkrootkit", "rkhunter"])
        .add_system_command(
            "2",
            "Run CHKRootKit Scan",
            "chkrootkit",
            "Scanning for rootkits with CHKRootKit"
        )
        .add_system_command(
            "3",
            "Run RKHunter Scan",
            "rkhunter --check --skip-keypress",
            "Scanning for rootkits with RKHunter"
        )
        .add_separator()
        .add_back("0", pages::page_1::generate_page)
        .display();
}
