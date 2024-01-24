use std::{io::stdout, error::Error};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, ExecutableCommand, event::{Event, self, KeyEventKind, KeyCode}};
use model::{Model, Action, update};
use ratatui::{Terminal, backend::CrosstermBackend};
use ui::view;

mod api;
mod overview;
mod thread;
mod model;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut model = Model::new();

    loop {
        terminal.draw(|frame| {
            view(&model, frame);
        })?;
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                let m = match key.code {
                    KeyCode::Up => Action::PrevThread,
                    KeyCode::Down => Action::NextThread,
                    KeyCode::PageUp => Action::ScrollUp,
                    KeyCode::PageDown => Action::ScrollDown,
                    KeyCode::Char('n') => Action::NextComment,
                    KeyCode::Char('p') => Action::PrevComment,
                    KeyCode::Char('q') => Action::Quit,
                    _ => Action::Nothing,
                };
                if m == Action::Quit {
                    break
                }

                update(&mut model, m)?;
            }
         }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
