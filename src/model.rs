use std::{error::Error, io::Stdout};

use ratatui::{backend::CrosstermBackend, Terminal};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{
    api::{get_thread, get_threads},
    overview::ThreadOverview,
    thread::ThreadData,
};

#[derive(Default, Deserialize, Serialize)]
pub struct Model {
    pub overview: Vec<ThreadOverview>,
    pub selected_thread: u16,
    pub overview_page: u16,
    pub data: ThreadsModel,
    pub viewer_scroll: u16,
    pub multiplier: Vec<u32>,
    #[serde(skip_serializing, skip_deserializing)]
    pub http_client: Client,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ThreadsModel {
    pub data: Vec<ThreadData>,
    pub selected_comment: u16,
}

#[derive(PartialEq)]
pub enum TabState {
    Home,
    Bookmarks,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
    NextThread,
    PrevThread,
    NextComment,
    PrevComment,
    ScrollDown,
    ScrollUp,
    Quit,
    Nothing,
    Moltiply(u32),
    Nullify,
    CleanComments,
}

impl Model {
    pub fn add_bookmark(&mut self, over: &ThreadOverview, data: &ThreadData) {
        if self.overview.iter().any(|x| x.url == over.url) {
            return;
        }
        self.overview.push(over.clone());
        self.data.data.push(data.clone());
    }

    pub fn delete_bookmark(&mut self, over: &ThreadOverview) {
        if let Some(n) = self.overview.iter().position(|x| x == over) {
            self.overview.remove(n);
            self.data.data.remove(n);
            if self.selected_thread > 0 {
                self.selected_thread -= 1;
            }
        }
    }

    pub fn clean_comments(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let thread = self
            .data
            .data
            .get_mut(self.selected_thread as usize)
            .unwrap();
        thread.comments.clear();
        self.data.selected_comment = 0 - 1;
        thread.comment_page = 0;
        self.next_comment(terminal)?;
        return Ok(());
    }

    fn next_thread(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        tab: &TabState,
    ) -> Result<(), Box<dyn Error>> {
        if *tab == TabState::Bookmarks && self.overview.is_empty() {
            return Ok(());
        }

        self.selected_thread += 1;
        while *tab == TabState::Home && self.selected_thread as usize >= self.overview.len() {
            self.overview_page += 1;
            let mut new_overviews =
                get_threads(&self.http_client, self.overview_page, terminal, true)?;
            self.overview.append(&mut new_overviews);
        }

        if self.selected_thread as usize >= self.overview.len() {
            self.selected_thread = self.overview.len() as u16 - 1;
        }

        self.viewer_scroll = 0;
        self.data.selected_comment = 0;
        if self.selected_thread as usize >= self.data.data.len() {
            let t_over = self.overview.get(self.selected_thread as usize).unwrap();
            let mut t = get_thread(&self.http_client, t_over, 1, terminal, true)?;
            t.comment_page = 1;
            self.data.data.push(t);
        }

        let t = self
            .data
            .data
            .get_mut(self.selected_thread as usize)
            .unwrap();
        while self.data.selected_comment as usize >= t.comments.len() {
            t.comment_page += 1;
            let t_over = self.overview.get(self.selected_thread as usize).unwrap();
            let mut new_comments =
                get_thread(&self.http_client, t_over, t.comment_page, terminal, true)?;
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

    fn next_comment(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        self.data.selected_comment += 1;
        self.viewer_scroll = 0;
        let t = self
            .data
            .data
            .get_mut(self.selected_thread as usize)
            .unwrap();
        while self.data.selected_comment as usize >= t.comments.len() {
            t.comment_page += 1;
            let t_over = self.overview.get(self.selected_thread as usize).unwrap();
            let mut new_comments =
                get_thread(&self.http_client, t_over, t.comment_page, terminal, true)?;
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

    pub(crate) fn new(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Self {
        let mut m = Model {
            ..Default::default()
        };
        m.overview = get_threads(&m.http_client, 1, terminal, false).unwrap();
        m.overview_page = 1;
        let t = get_thread(
            &m.http_client,
            m.overview.get(m.selected_thread as usize).unwrap(),
            1,
            terminal,
            false,
        )
        .unwrap();
        m.data.data.push(t);
        return m;
    }

    pub fn new_bookmarks() -> Self {
        Model {
            ..Default::default()
        }
    }

    fn add_multiplier(&mut self, n: u32) -> Result<(), Box<dyn Error>> {
        self.multiplier.push(n);
        return Ok(());
    }

    fn get_multiplier(&mut self) -> u32 {
        if self.multiplier.is_empty() {
            return 1;
        }

        let mut ris = 0;
        while !self.multiplier.is_empty() {
            let m = self.multiplier.remove(0);
            ris += (10_u32).pow(self.multiplier.len() as u32) * m;
        }
        return ris;
    }

    fn clean_multiplier(&mut self) -> Result<(), Box<dyn Error>> {
        self.multiplier.clear();
        return Ok(());
    }
}

pub fn update(
    model: &mut Model,
    action: Action,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    tab: &TabState,
) -> Result<(), Box<dyn Error>> {
    match action {
        Action::Quit | Action::Nothing | Action::Moltiply(_) | Action::Nullify => {
            let _ = match action {
                Action::Quit => unreachable!(),
                Action::Nothing => Ok(()),
                Action::Moltiply(n) => model.add_multiplier(n),
                Action::Nullify => model.clean_multiplier(),
                _ => Ok(()),
            };
            return Ok(());
        }
        _ => (),
    };

    let mult = model.get_multiplier();

    for _ in 0..mult {
        match action {
            Action::NextThread => model.next_thread(terminal, tab),
            Action::PrevThread => model.prev_thread(),
            Action::NextComment => model.next_comment(terminal),
            Action::PrevComment => model.prev_comment(),
            Action::ScrollDown => model.scroll_down(),
            Action::ScrollUp => model.scroll_up(),
            Action::CleanComments => model.clean_comments(terminal),
            _ => unreachable!(),
        }?;
    }
    return Ok(());
}
