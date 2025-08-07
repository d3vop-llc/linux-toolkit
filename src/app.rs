use anyhow::Result;
use crossterm::{ event::{ self, Event, KeyCode, KeyEventKind } };
use ratatui::{
    backend::CrosstermBackend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Line, Span },
    widgets::{ Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap },
    Frame,
    Terminal,
};
use std::io;
use tokio::time::Duration;

use crate::commands::{ Command, CommandCategory };
use crate::config::Config;

pub struct App {
    pub config: Config,
    pub should_quit: bool,
    pub categories: Vec<CommandCategory>,
    pub current_category: usize,
    pub current_command: usize,
    pub category_list_state: ListState,
    pub command_list_state: ListState,
    pub focused_panel: FocusedPanel,
    pub show_help: bool,
    pub show_command_details: bool,
    pub executing_command: bool,
    pub command_output: Vec<String>,
    pub input_mode: bool,
    pub input_buffer: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusedPanel {
    Categories,
    Commands,
    Details,
}

impl App {
    pub fn new(config: Config) -> Self {
        let categories = crate::commands::load_categories();
        let mut category_list_state = ListState::default();
        let mut command_list_state = ListState::default();

        // Select first items by default
        if !categories.is_empty() {
            category_list_state.select(Some(0));
            if !categories[0].commands.is_empty() {
                command_list_state.select(Some(0));
            }
        }

        Self {
            config,
            should_quit: false,
            categories,
            current_category: 0,
            current_command: 0,
            category_list_state,
            command_list_state,
            focused_panel: FocusedPanel::Categories,
            show_help: false,
            show_command_details: false,
            executing_command: false,
            command_output: Vec::new(),
            input_mode: false,
            input_buffer: String::new(),
        }
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>
    ) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key.code).await?;
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, key: KeyCode) -> Result<()> {
        if self.input_mode {
            match key {
                KeyCode::Enter => {
                    // Process input
                    self.input_mode = false;
                    self.input_buffer.clear();
                }
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.input_buffer.clear();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            }
            return Ok(());
        }

        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('h') | KeyCode::F(1) => {
                self.show_help = !self.show_help;
            }
            KeyCode::Tab => {
                self.cycle_focus();
            }
            KeyCode::Enter => {
                if self.focused_panel == FocusedPanel::Commands {
                    self.execute_selected_command().await?;
                }
            }
            KeyCode::Char(' ') => {
                if self.focused_panel == FocusedPanel::Commands {
                    self.show_command_details = !self.show_command_details;
                }
            }
            KeyCode::Up => {
                self.move_up();
            }
            KeyCode::Down => {
                self.move_down();
            }
            KeyCode::Left => {
                if self.focused_panel == FocusedPanel::Commands {
                    self.focused_panel = FocusedPanel::Categories;
                }
            }
            KeyCode::Right => {
                if self.focused_panel == FocusedPanel::Categories {
                    self.focused_panel = FocusedPanel::Commands;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn cycle_focus(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::Categories => FocusedPanel::Commands,
            FocusedPanel::Commands => FocusedPanel::Details,
            FocusedPanel::Details => FocusedPanel::Categories,
        };
    }

    fn move_up(&mut self) {
        match self.focused_panel {
            FocusedPanel::Categories => {
                if self.current_category > 0 {
                    self.current_category -= 1;
                    self.category_list_state.select(Some(self.current_category));
                    self.update_commands_for_category();
                }
            }
            FocusedPanel::Commands => {
                if self.current_command > 0 {
                    self.current_command -= 1;
                    self.command_list_state.select(Some(self.current_command));
                }
            }
            _ => {}
        }
    }

    fn move_down(&mut self) {
        match self.focused_panel {
            FocusedPanel::Categories => {
                if self.current_category < self.categories.len().saturating_sub(1) {
                    self.current_category += 1;
                    self.category_list_state.select(Some(self.current_category));
                    self.update_commands_for_category();
                }
            }
            FocusedPanel::Commands => {
                if let Some(category) = self.categories.get(self.current_category) {
                    if self.current_command < category.commands.len().saturating_sub(1) {
                        self.current_command += 1;
                        self.command_list_state.select(Some(self.current_command));
                    }
                }
            }
            _ => {}
        }
    }

    fn update_commands_for_category(&mut self) {
        self.current_command = 0;
        self.command_list_state.select(Some(0));
    }

    async fn execute_selected_command(&mut self) -> Result<()> {
        if let Some(category) = self.categories.get(self.current_category) {
            if let Some(command) = category.commands.get(self.current_command) {
                self.executing_command = true;
                self.command_output.clear();

                // Execute the command
                match crate::commands::execute_command(command, &self.config).await {
                    Ok(output) => {
                        self.command_output = output
                            .lines()
                            .map(|s| s.to_string())
                            .collect();
                    }
                    Err(e) => {
                        self.command_output.push(format!("Error: {}", e));
                    }
                }

                self.executing_command = false;
                self.show_command_details = true;
            }
        }
        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(35),
                Constraint::Percentage(40),
            ])
            .split(f.size());

        self.render_categories(f, chunks[0]);
        self.render_commands(f, chunks[1]);
        self.render_details(f, chunks[2]);

        if self.show_help {
            self.render_help_popup(f);
        }
    }

    fn render_categories(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.categories
            .iter()
            .map(|category| {
                ListItem::new(
                    Line::from(Span::styled(&category.name, Style::default().fg(Color::White)))
                )
            })
            .collect();

        let border_style = if self.focused_panel == FocusedPanel::Categories {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Categories")
                    .border_style(border_style)
            )
            .highlight_style(
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            );

        f.render_stateful_widget(list, area, &mut self.category_list_state);
    }

    fn render_commands(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = if
            let Some(category) = self.categories.get(self.current_category)
        {
            category.commands
                .iter()
                .map(|command| {
                    ListItem::new(
                        Line::from(
                            vec![
                                Span::styled(&command.name, Style::default().fg(Color::White)),
                                Span::styled(
                                    format!(" - {}", command.description),
                                    Style::default().fg(Color::Gray)
                                )
                            ]
                        )
                    )
                })
                .collect()
        } else {
            vec![]
        };

        let border_style = if self.focused_panel == FocusedPanel::Commands {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        let list = List::new(items)
            .block(
                Block::default().borders(Borders::ALL).title("Commands").border_style(border_style)
            )
            .highlight_style(
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            );

        f.render_stateful_widget(list, area, &mut self.command_list_state);
    }

    fn render_details(&mut self, f: &mut Frame, area: Rect) {
        let border_style = if self.focused_panel == FocusedPanel::Details {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        if self.executing_command {
            let paragraph = Paragraph::new("Executing command...")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Status")
                        .border_style(border_style)
                )
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(paragraph, area);
            return;
        }

        if !self.command_output.is_empty() {
            let output_text = self.command_output.join("\n");
            let paragraph = Paragraph::new(output_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Output")
                        .border_style(border_style)
                )
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Color::White));
            f.render_widget(paragraph, area);
            return;
        }

        if let Some(category) = self.categories.get(self.current_category) {
            if let Some(command) = category.commands.get(self.current_command) {
                let mut text = vec![
                    Line::from(
                        vec![
                            Span::styled("Name: ", Style::default().fg(Color::Green)),
                            Span::styled(&command.name, Style::default().fg(Color::White))
                        ]
                    ),
                    Line::from(""),
                    Line::from(
                        vec![
                            Span::styled("Description: ", Style::default().fg(Color::Green)),
                            Span::styled(&command.description, Style::default().fg(Color::White))
                        ]
                    ),
                    Line::from("")
                ];

                if !command.usage.is_empty() {
                    text.push(
                        Line::from(
                            vec![
                                Span::styled("Usage: ", Style::default().fg(Color::Green)),
                                Span::styled(&command.usage, Style::default().fg(Color::Cyan))
                            ]
                        )
                    );
                    text.push(Line::from(""));
                }

                if !command.tags.is_empty() {
                    text.push(
                        Line::from(
                            vec![
                                Span::styled("Tags: ", Style::default().fg(Color::Green)),
                                Span::styled(
                                    command.tags.join(", "),
                                    Style::default().fg(Color::Yellow)
                                )
                            ]
                        )
                    );
                    text.push(Line::from(""));
                }

                text.push(Line::from(""));
                text.push(
                    Line::from(
                        vec![
                            Span::styled("Press ", Style::default().fg(Color::Gray)),
                            Span::styled("Enter", Style::default().fg(Color::Green)),
                            Span::styled(" to execute", Style::default().fg(Color::Gray))
                        ]
                    )
                );

                let paragraph = Paragraph::new(text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Details")
                            .border_style(border_style)
                    )
                    .wrap(Wrap { trim: true });
                f.render_widget(paragraph, area);
            }
        }
    }

    fn render_help_popup(&self, f: &mut Frame) {
        let popup_area = self.centered_rect(60, 70, f.size());

        f.render_widget(Clear, popup_area);

        let help_text = vec![
            Line::from(
                vec![
                    Span::styled(
                        "Linux Toolkit - Help",
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    )
                ]
            ),
            Line::from(""),
            Line::from(vec![Span::styled("Navigation:", Style::default().fg(Color::Yellow))]),
            Line::from("  ↑/↓        - Navigate lists"),
            Line::from("  ←/→        - Switch between panels"),
            Line::from("  Tab        - Cycle through panels"),
            Line::from("  Enter      - Execute selected command"),
            Line::from("  Space      - Toggle command details"),
            Line::from(""),
            Line::from(vec![Span::styled("General:", Style::default().fg(Color::Yellow))]),
            Line::from("  h/F1       - Toggle this help"),
            Line::from("  q/Esc      - Quit application"),
            Line::from(""),
            Line::from(vec![Span::styled("Tips:", Style::default().fg(Color::Yellow))]),
            Line::from("  • Commands are organized by category"),
            Line::from("  • Green highlights indicate focus"),
            Line::from("  • Output appears in the details panel"),
            Line::from("  • Some commands may require sudo"),
            Line::from(""),
            Line::from("Press h or F1 to close this help")
        ];

        let paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .border_style(Style::default().fg(Color::Green))
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, popup_area);
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
