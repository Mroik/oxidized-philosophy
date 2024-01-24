use std::error::Error;

use crate::{overview::ThreadOverview, thread::ThreadData, api::{get_threads, get_thread}};

#[derive(Default)]
pub struct Model {
    pub overview: Vec<ThreadOverview>,
    pub selected_thread: u16,
    pub overview_page: u16,
    pub data: ThreadsModel,
    pub viewer_scroll: u16,
}

#[derive(Default)]
pub struct ThreadsModel {
    pub data: Vec<ThreadData>,
    pub selected_comment: u16,
}

#[derive(PartialEq)]
pub enum Action {
    NextThread,
    PrevThread,
    NextComment,
    PrevComment,
    ScrollDown,
    ScrollUp,
    Quit,
    Nothing,
}

impl Model {
    fn next_thread(&mut self) -> Result<(), Box<dyn Error>> {
        self.selected_thread += 1;
        while self.selected_thread as usize >= self.overview.len() {
            self.overview_page += 1;
            let mut new_overviews = get_threads(self.overview_page)?;
            self.overview.append(&mut new_overviews);
        }

        self.viewer_scroll = 0;
        self.data.selected_comment = 0;
        if self.selected_thread as usize >= self.data.data.len() {
            let t_over = self.overview.get(self.selected_thread as usize).unwrap();
            let mut t = get_thread(t_over, 1).unwrap();
            t.comment_page = 1;
            self.data.data.push(t);
        }

        let mut t = self.data.data.get_mut(self.selected_thread as usize).unwrap();
        while self.data.selected_comment as usize >= t.comments.len() {
            t.comment_page += 1;
            let t_over = self.overview.get(self.selected_thread as usize).unwrap();
            let mut new_comments = get_thread(t_over, t.comment_page).unwrap();
            t.comments.append(&mut new_comments.comments);
        }

        return Ok(());
    }

    fn prev_thread(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_thread == 0 {
            return Ok(());
        }
        self.selected_thread -= 1;
        self.viewer_scroll = 0;
        self.data.selected_comment = 0;
        return Ok(());
    }

    fn next_comment(&mut self) -> Result<(), Box<dyn Error>> {
        self.data.selected_comment += 1;
        self.viewer_scroll = 0;
        let mut t = self.data.data.get_mut(self.selected_thread as usize).unwrap();
        while self.data.selected_comment as usize >= t.comments.len() {
            t.comment_page += 1;
            let t_over = self.overview.get(self.selected_thread as usize).unwrap();
            let mut new_comments = get_thread(t_over, t.comment_page).unwrap();
            t.comments.append(&mut new_comments.comments);
        }
        return Ok(());
    }

    fn prev_comment(&mut self) -> Result<(), Box<dyn Error>> {
        if self.data.selected_comment == 0 {
            return Ok(());
        }
        self.data.selected_comment -= 1;
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

    pub(crate) fn new() -> Model {
        let mut m = Model { ..Default::default() };
        m.overview = get_threads(1).unwrap();
        m.overview_page = 1;
        let t = get_thread(m.overview.get(m.selected_thread as usize).unwrap(), 1).unwrap();
        m.data.data.push(t);
        return m;
    }
}

pub fn update(model: &mut Model, action: Action) -> Result<(), Box<dyn Error>> {
    match action {
        Action::NextThread => model.next_thread(),
        Action::PrevThread => model.prev_thread(),
        Action::NextComment => model.next_comment(),
        Action::PrevComment => model.prev_comment(),
        Action::ScrollDown => model.scroll_down(),
        Action::ScrollUp => model.scroll_up(),
        Action::Quit => panic!(),
        Action::Nothing => Ok(()),
    }
}
