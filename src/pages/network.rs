use crate::utils::page_builder::PageBuilder;
use crate::pages;

pub fn generate_page() {
    PageBuilder::new("Network Tools")
        .add_apt_install("1", "Install Network Tools", &["net-tools", "nmap", "netstat-nat"])
        .add_system_command(
            "2",
            "Show Network Interfaces",
            "ip addr show",
            "Displaying network interfaces"
        )
        .add_system_command(
            "3",
            "Show Network Connections",
            "netstat -tuln",
            "Displaying network connections"
        )
        .add_system_command(
            "4",
            "Scan Local Network",
            "nmap -sn 192.168.1.0/24",
            "Scanning local network"
        )
        .add_system_command(
            "5",
            "Check Internet Connectivity",
            "ping -c 4 8.8.8.8",
            "Testing internet connectivity"
        )
        .add_separator()
        .add_back("0", pages::page_1::generate_page)
        .display();
}
