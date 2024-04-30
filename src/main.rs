use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    layout::{Constraint, Direction, Layout},
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let options = vec!["Record Weight", "Record Blood Pressure", "Quit"];
    let mut selected_option = 0;
    let mut list_state = ListState::default();
    list_state.select(Some(selected_option));

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(area);

            let welcome_text = Paragraph::new("Welcome to Health Tracker!")
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Welcome"));
            frame.render_widget(welcome_text, chunks[0]);

            let options_list = List::new(
                options
                    .iter()
                    .map(|option| ListItem::new(option.to_string()))
                    .collect::<Vec<ListItem>>(),
            )
            .block(Block::default().borders(Borders::ALL).title("Options"))
            .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
            .highlight_symbol("> ");
            frame.render_stateful_widget(options_list, chunks[1], &mut list_state);

            let instruction_text = Paragraph::new("Use 'j' and 'k' to navigate, 'Enter' to select.")
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Gray))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(instruction_text, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('j') => {
                        if selected_option < options.len() - 1 {
                            selected_option += 1;
                            list_state.select(Some(selected_option));
                        }
                    }
                    KeyCode::Char('k') => {
                        if selected_option > 0 {
                            selected_option -= 1;
                            list_state.select(Some(selected_option));
                        }
                    }
                    KeyCode::Enter => {
                        match selected_option {
                            0 => {
                                // TODO: Implement weight recording functionality
                                break;
                            }
                            1 => {
                                // TODO: Implement blood pressure recording functionality
                                break;
                            }
                            2 => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
