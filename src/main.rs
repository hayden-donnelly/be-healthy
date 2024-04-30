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
    widgets::{Block, Borders, Paragraph},
    layout::{Constraint, Direction, Layout},
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

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

            let options_text = Paragraph::new(
                "Please select an option:\n\n1. Record Weight\n2. Record Blood Pressure\n3. Quit",
            )
            .alignment(ratatui::layout::Alignment::Left)
            .block(Block::default().borders(Borders::ALL).title("Options"));
            frame.render_widget(options_text, chunks[1]);

            let instruction_text = Paragraph::new("Press the corresponding number key to select an option.")
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Gray))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(instruction_text, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('1') => {
                        // TODO: Implement weight recording functionality
                        break;
                    }
                    KeyCode::Char('2') => {
                        // TODO: Implement blood pressure recording functionality
                        break;
                    }
                    KeyCode::Char('3') => {
                        break;
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
