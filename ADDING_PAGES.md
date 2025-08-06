# Adding New Pages and Commands

This document explains how to easily add new pages, commands, and functionality to the Linux Toolkit using the new extensible menu system.

## Quick Start: Adding a New Page

To add a new page, you only need to create a new file in `src/pages/` and follow this simple pattern:

```rust
use crate::utils::page_builder::PageBuilder;
use crate::pages;

pub fn generate_page() {
    PageBuilder::new("Your Page Title")
        .add_system_command("1", "Command Name", "bash_command", "Description")
        .add_apt_install("2", "Install Tools", &["package1", "package2"])
        .add_page("3", "Sub Page", sub_page_function)
        .add_separator()
        .add_back("0", pages::page_1::generate_page)
        .display();
}
```

Then:

1. Add `pub mod your_page;` to `src/pages/mod.rs`
2. Add the page to a parent menu using `.add_page("X", "Page Name", your_page::generate_page)`

## Available PageBuilder Methods

### Navigation

- `.add_page(number, text, function)` - Link to another page/menu
- `.add_back(number, back_function)` - Return to previous menu
- `.add_exit(number)` - Exit the application

### Commands

- `.add_system_command(number, text, command, description)` - Run a shell command with sudo
- `.add_apt_install(number, text, packages)` - Install apt packages
- `.add_custom_command(number, text, sudo, args, start_msg, success_msg, error_msg)` - Full control over command

### Layout

- `.add_separator()` - Add empty line for visual separation

## Examples

### Simple System Tools Page

```rust
pub fn system_tools_page() {
    PageBuilder::new("System Tools")
        .add_system_command("1", "Show Disk Usage", "df -h", "Checking disk usage")
        .add_system_command("2", "Show Memory Usage", "free -h", "Checking memory usage")
        .add_system_command("3", "Show Running Processes", "ps aux", "Listing processes")
        .add_separator()
        .add_back("0", pages::page_1::generate_page)
        .display();
}
```

### Package Management Page

```rust
pub fn package_management_page() {
    PageBuilder::new("Package Management")
        .add_apt_install("1", "Install Development Tools", &["build-essential", "git", "vim"])
        .add_apt_install("2", "Install Media Tools", &["ffmpeg", "vlc", "gimp"])
        .add_system_command("3", "Update Package Lists", "apt update", "Updating package lists")
        .add_system_command("4", "Upgrade Packages", "apt upgrade -y", "Upgrading packages")
        .add_separator()
        .add_back("0", pages::page_1::generate_page)
        .display();
}
```

### Multi-level Navigation

```rust
pub fn main_menu() {
    PageBuilder::new("Main Menu")
        .add_page("1", "System Tools", system_tools_page)
        .add_page("2", "Package Management", package_management_page)
        .add_page("3", "Security Tools", security_page)
        .add_separator()
        .add_exit("0")
        .display();
}
```

## Advanced: Custom Commands

For more complex commands, use `add_custom_command`:

```rust
.add_custom_command(
    "5",
    "Custom Backup",
    true, // needs sudo
    vec!["sh", "-c", "tar -czf /backup/system-$(date +%Y%m%d).tar.gz /home /etc"],
    "Creating system backup...",
    "Backup completed successfully",
    "Backup failed"
)
```

## File Structure

When adding a new page:

1. Create `src/pages/your_page.rs`
2. Add `pub mod your_page;` to `src/pages/mod.rs`
3. Reference it in parent menus as `pages::your_page::generate_page`

## Color Theming

The system automatically uses consistent colors defined in `src/variables/colors.rs`:

- Menu titles: White on green background
- Option numbers: Light blue
- Option text: White
- Prompts: Light grey
- Error messages: Red
- Success messages: Green

## Benefits of This System

1. **Consistent UI**: All menus look and behave the same
2. **Error Handling**: Automatic input validation and error display
3. **Easy Navigation**: Back buttons and exit options handled automatically
4. **Command Integration**: Built-in command execution with status messages
5. **Maintainable**: Adding new functionality requires minimal code
6. **Type Safe**: Compile-time checking prevents common errors

## Migration from Old System

Old pattern:

```rust
// 50+ lines of repetitive menu handling code
let options = vec![...];
let mut input = String::new();
io::stdin().read_line(&mut input).expect(...);
match input.trim().parse::<u8>() {
    // More repetitive code...
}
```

New pattern:

```rust
// 10 lines, no boilerplate
PageBuilder::new("Page Title")
    .add_system_command("1", "Command", "bash_cmd", "Description")
    .add_back("0", parent_function)
    .display();
```

This reduces code by ~80% while making it more maintainable and consistent.
