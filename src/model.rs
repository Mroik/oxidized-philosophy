use std::error::Error;

use crate::{overview::ThreadOverview, thread::ThreadData, api::{get_threads, get_thread}};

struct Model {
    overview: Vec<ThreadOverview>,
    selected: u16,
    page: u16,
    data: ThreadsModel,
    viewer_scroll: u16,
}

struct ThreadsModel {
    data: Vec<ThreadData>,
    selected: u16,
    page: u16,
}

enum Action {
    NextThread,
    PrevThread,
    NextComment,
    PrevComment,
    ScrollDown,
    ScrollUp,
    Quit,
}

impl Model {
    fn next_thread(&mut self) -> Result<(), Box<dyn Error>> {
        self.selected += 1;
        while self.selected as usize >= self.overview.len() {
            self.page += 1;
            let mut new_overviews = get_threads(self.page)?;
            self.overview.append(&mut new_overviews);
        }

        self.viewer_scroll = 0;
        self.data.selected = 0;
        while self.data.selected as usize >= self.data.data.len() {
            self.data.page += 1;
            let t = self.overview.get(self.selected as usize).unwrap();
            let new_thread = get_thread(t)?;
            self.data.data.push(new_thread);
        }

        return Ok(());
    }

    fn prev_thread(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected == 0 {
            return Ok(());
        }
        self.selected -= 1;
        self.viewer_scroll = 0;
        self.data.selected = 0;
        return Ok(());
    }

    fn next_comment(&mut self) -> Result<(), Box<dyn Error>> {
        self.data.selected += 1;
        self.viewer_scroll = 0;
        while self.data.selected as usize >= self.data.data.len() {
            self.data.page += 1;
            let t = self.overview.get(self.selected as usize).unwrap();
            let new_thread = get_thread(t)?;
            self.data.data.push(new_thread);
        }
        return Ok(());
    }

    fn prev_comment(&mut self) -> Result<(), Box<dyn Error>> {
        if self.data.selected == 0 {
            return Ok(());
        }
        self.data.selected -= 1;
        self.viewer_scroll = 0;
        return Ok(());
    }

    fn scroll_down(&mut self) -> Result<(), Box<dyn Error>> {
        self.viewer_scroll += 1;
        return Ok(());
    }

    fn scroll_up(&mut self) -> Result<(), Box<dyn Error>> {
        if self.viewer_scroll > 0 {
            self.viewer_scroll -= 1;
        }
        return Ok(());
    }
}

fn update(model: &mut Model, action: Action) -> Result<(), Box<dyn Error>> {
    match action {
        Action::NextThread => model.next_thread(),
        Action::PrevThread => model.prev_thread(),
        Action::NextComment => model.next_comment(),
        Action::PrevComment => model.prev_comment(),
        Action::ScrollDown => model.scroll_down(),
        Action::ScrollUp => model.scroll_up(),
        Action::Quit => panic!(),
    }
}
