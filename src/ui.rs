use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListState, Paragraph, Row, Table, TableState, Wrap},
    Frame, Terminal,
};

use crate::{model::Model, thread::ThreadData};

fn generate_layout(frame: &Frame) -> (Rect, Rect, Rect, Rect) {
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(root[1]);
    let info_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size());
    return (root[0], right[0], right[1], info_area[1]);
}

pub fn view(model: &Model, frame: &mut Frame) {
    let (overview, comments, viewer, _) = generate_layout(frame);
    let thread = if model.overview.len() == 0 {
        None
    } else {
        Some(model.data.data.get(model.selected_thread as usize).unwrap())
    };

    render_overview(model, frame, overview);
    render_comment_list(thread, model, frame, comments);
    render_viwer(thread, model, frame, viewer);
}

fn render_viwer(thread: Option<&ThreadData>, model: &Model, frame: &mut Frame, area: Rect) {
    fn generate_paragraph(text: Vec<Line>, title: String, offset: u16) -> Paragraph {
        Paragraph::new(text)
            .block(
                Block::new()
                    .title(title)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Red)),
            )
            .style(Style::new().white())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .scroll((offset, 0))
    }

    let (text, title, offset) = if thread.is_some() {
        let t = thread.unwrap();
        let comment = t
            .comments
            .get(model.data.selected_comment as usize)
            .unwrap();
        (comment.get_lines(), t.title.clone(), model.viewer_scroll)
    } else {
        (vec![], "".to_string(), 0)
    };

    let parag = generate_paragraph(text, title, offset);
    frame.render_widget(parag, area);
}

fn render_comment_list(thread: Option<&ThreadData>, model: &Model, frame: &mut Frame, area: Rect) {
    let rows = if let Some(t) = thread {
        t.comments
            .iter()
            .map(|x| Row::new(vec![x.author.as_str(), x.date.as_str()]))
            .collect()
    } else {
        vec![]
    };

    let widths = [Constraint::Percentage(30), Constraint::Percentage(50)];
    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .title("Comments")
                .style(Style::default().fg(Color::Red))
                .borders(Borders::ALL),
        )
        .highlight_style(Style::new().bg(Color::LightBlue))
        .highlight_symbol(">>");

    let mut state = TableState::default();
    let s = if thread.is_some() {
        Some(model.data.selected_comment.into())
    } else {
        None
    };
    state.select(s);
    frame.render_stateful_widget(table, area, &mut state);
}

fn render_overview(model: &Model, frame: &mut Frame, area: Rect) {
    let threads_list = List::new(model.overview.iter().map(|item| item.title.clone()))
        .block(
            Block::default()
                .title("Overview")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bg(Color::LightBlue))
        .highlight_symbol(">>");
    let mut state = ListState::default();
    let s = if model.overview.len() == 0 {
        None
    } else {
        Some(model.selected_thread.into())
    };
    state.select(s);
    frame.render_stateful_widget(threads_list, area, &mut state);
}

pub fn print_info(terminal: &mut Terminal<CrosstermBackend<Stdout>>, text: &str) {
    let _ = terminal.draw(|frame| {
        let (_, _, _, info_area) = generate_layout(frame);
        let parag = Paragraph::new(text)
            .style(Style::default().yellow())
            .alignment(Alignment::Left);
        frame.render_widget(parag, info_area);
    });
}
