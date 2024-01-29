#![allow(clippy::needless_return, arithmetic_overflow)]
use std::{
    error::Error,
    fs::{self, File},
    io::stdout,
};

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

    // TODO Remove hard-coded path
    let reader = fs::File::open("/home/mroik/.cache/oxi-phil/bookmarks.txt");
    let data = if reader.is_ok() {
        serde_cbor::from_reader(reader?)?
    } else {
        Model::new_bookmarks()
    };

    // TODO Instead of having 2 models make a bookmark struct within model
    let mut model = [Model::new(&mut terminal), data];
    let mut tab = TabState::Home;
    let mut running = true;

    while running {
        let current_model = match tab {
            TabState::Home => &model[0],
            TabState::Bookmarks => &model[1],
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
                    KeyCode::Char('c') => Action::CleanComments,
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
                    /* TODO Move logic into model when bookmarks
                     * will have a struct within model itself
                     */
                    KeyCode::Char('b') => {
                        if tab == TabState::Home {
                            let mm = model.get(0).unwrap();
                            let over = mm
                                .overview
                                .get(mm.selected_thread as usize)
                                .unwrap()
                                .clone();
                            let data = mm
                                .data
                                .data
                                .get(mm.selected_thread as usize)
                                .unwrap()
                                .clone();
                            model.get_mut(1).unwrap().add_bookmark(&over, &data);
                        }
                        Action::Nothing
                    }
                    KeyCode::Char('u') => {
                        let mm = model.get(0).unwrap();
                        let over = mm
                            .overview
                            .get(mm.selected_thread as usize)
                            .unwrap()
                            .clone();
                        model.get_mut(1).unwrap().delete_bookmark(&over);
                        Action::Nothing
                    }
                    _ => Action::Nothing,
                };

                if m == Action::Quit {
                    running = false;
                } else {
                    let current_model = match tab {
                        TabState::Home => &mut model[0],
                        TabState::Bookmarks => &mut model[1],
                    };
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
    let file = File::create("/home/mroik/.cache/oxi-phil/bookmarks.txt")?;
    serde_cbor::to_writer(file, &model[1])?;
    println!("done");
    return Ok(());
}
