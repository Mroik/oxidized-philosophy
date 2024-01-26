use std::{error::Error, io::stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use model::{update, Action, Model};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::view;

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

    let mut model = Model::new(&mut terminal);
    let mut running = true;

    while running {
        terminal.draw(|frame| view(&model, frame))?;
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
                    KeyCode::Char(n) if n >= '0' && n <= '9' => {
                        Action::Moltiply(n.to_digit(10).unwrap())
                    }
                    KeyCode::Esc => Action::Nullify,
                    _ => Action::Nothing,
                };

                if m == Action::Quit {
                    running = false;
                } else {
                    update(&mut model, m, &mut terminal).unwrap();
                    if m != Action::Nothing {
                        terminal.draw(|frame| view(&model, frame))?;
                    }
                }
            }
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(());
}
