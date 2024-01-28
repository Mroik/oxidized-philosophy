#![allow(clippy::needless_return)]
use std::{error::Error, fs::File, io::stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use model::{update, Action, Model};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::view;

use crate::model::TabState;

mod api;
mod model;
mod overview;
mod thread;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut model = [Model::new(&mut terminal), Model::new_bookmarks()];
    let mut tab = TabState::Home;
    let mut running = true;

    while running {
        let current_model = match tab {
            TabState::Home => &mut model[0],
            TabState::Bookmarks => &mut model[1],
        };
        terminal.draw(|frame| view(current_model, frame))?;
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                let m = match key.code {
                    KeyCode::Up => Action::PrevThread,
                    KeyCode::Down => Action::NextThread,
                    KeyCode::PageUp => Action::ScrollUp,
                    KeyCode::PageDown => Action::ScrollDown,
                    KeyCode::Char('n') | KeyCode::Right => Action::NextComment,
                    KeyCode::Char('p') | KeyCode::Left => Action::PrevComment,
                    KeyCode::Char('q') => Action::Quit,
                    KeyCode::Char(n) if n.is_ascii_digit() => {
                        Action::Moltiply(n.to_digit(10).unwrap())
                    }
                    KeyCode::Esc => Action::Nullify,
                    KeyCode::Char('z') => {
                        tab = TabState::Home;
                        Action::Nothing
                    }
                    KeyCode::Char('x') => {
                        tab = TabState::Bookmarks;
                        Action::Nothing
                    }
                    _ => Action::Nothing,
                };

                if m == Action::Quit {
                    running = false;
                } else {
                    update(current_model, m, &mut terminal, &tab).unwrap();
                    if m != Action::Nothing {
                        terminal.draw(|frame| view(current_model, frame))?;
                    }
                }
            }
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    print!("Saving bookmarks... ");
    let file = File::create("bookmarks.txt")?;
    serde_cbor::to_writer(file, &model[1])?;
    println!("done");
    return Ok(());
}
