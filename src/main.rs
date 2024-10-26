use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::Layout;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::DefaultTerminal;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app = run(terminal);
    ratatui::restore();
    app
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let alert_message = "Alert: It's time to prepare Ratatouille!";
    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .margin(1)
                .constraints(vec![ratatui::layout::Constraint::Percentage(100)])
                .split(size);

            let alert_block = Block::default()
                .borders(Borders::ALL)
                .title("Ratatouille Alert");
            let alert_paragraph = Paragraph::new(alert_message).block(alert_block);

            frame.render_widget(alert_paragraph, chunks[0]);
        })?;
        // Break the loop on any key press
        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            if let crossterm::event::Event::Key(_) = crossterm::event::read()? {
                break Ok(());
            }
        }
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
