use crate::utils::page_builder::PageBuilder;
use crate::pages;

pub fn generate_page() {
    PageBuilder::new("Security")
        .add_system_command(
            "1",
            "Configure Firewall",
            "ufw enable && ufw default deny incoming && ufw default allow outgoing",
            "Configuring firewall"
        )
        .add_apt_install("2", "Install Fail2Ban", &["fail2ban"])
        .add_system_command(
            "3",
            "Enable Fail2Ban",
            "systemctl enable fail2ban && systemctl start fail2ban",
            "Enabling Fail2Ban"
        )
        .add_apt_install("4", "Install Security Audit Tools", &["chkrootkit", "rkhunter"])
        .add_system_command(
            "5",
            "Run Security Audit",
            "chkrootkit && rkhunter --check --skip-keypress",
            "Running security audit"
        )
        .add_separator()
        .add_back("0", pages::page_1::generate_page)
        .display();
}
