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
    Frame
};
use std::io::{stdout, Result};

enum Page {
    Main,
    WeightRecording,
    BloodPressureRecording,
}

fn render_main_page(frame: &mut Frame, selected_option: usize) {
    let options = vec!["Record Weight", "Record Blood Pressure", "Quit"];
    let mut list_state = ListState::default();
    list_state.select(Some(selected_option));

    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(75),
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
}

fn render_weight_recording_page(frame: &mut Frame) {
    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

    let text = Paragraph::new("This is the weight recording recording page.")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().title("Record Weight").borders(Borders::ALL));
    frame.render_widget(text, chunks[0]);
    
    let instruction_text = Paragraph::new("Press 'Esc' to go back.")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(instruction_text, chunks[1]);
}

fn render_blood_pressure_recording_page(frame: &mut Frame) {
    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

    let text = Paragraph::new("This is the blood pressure recording page.")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().title("Record Blood Pressure").borders(Borders::ALL));
    frame.render_widget(text, chunks[0]);
    
    let instruction_text = Paragraph::new("Press 'Esc' to go back.")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(instruction_text, chunks[1]);
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let options = vec!["Record Weight", "Record Blood Pressure", "Quit"];
    let mut selected_option = 0;

    let mut current_page = Page::Main;

    loop {
        terminal.draw(|frame| {
            match current_page {
                Page::Main => render_main_page(frame, selected_option),
                Page::WeightRecording => render_weight_recording_page(frame),
                Page::BloodPressureRecording => render_blood_pressure_recording_page(frame),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match current_page {
                    Page::Main => {
                        match key.code {
                            KeyCode::Char('j') => {
                                if selected_option < options.len() - 1 {
                                    selected_option += 1;
                                }
                            }
                            KeyCode::Char('k') => {
                                if selected_option > 0 {
                                    selected_option -= 1;
                                }
                            }
                            KeyCode::Enter => {
                                match selected_option {
                                    0 => {
                                        current_page = Page::WeightRecording;
                                    }
                                    1 => {
                                        current_page = Page::BloodPressureRecording;
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
                    Page::WeightRecording | Page::BloodPressureRecording => {
                        match key.code {
                            KeyCode::Esc => {
                                current_page = Page::Main;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
