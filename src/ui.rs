use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListState, Paragraph, Row, Table, TableState, Wrap},
    Frame,
};

use crate::{model::Model, thread::ThreadData};

fn generate_layout(frame: &Frame) -> (Rect, Rect, Rect) {
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(root[1]);
    return (root[0], right[0], right[1]);
}

pub fn view(model: &Model, frame: &mut Frame) {
    let (overview, comments, viewer) = generate_layout(frame);
    render_overview(model, frame, overview);
    let thread = model.data.data.get(model.selected_thread as usize).unwrap();
    render_comment_list(thread, model, frame, comments);
    render_viwer(thread, model, frame, viewer);
}

fn render_viwer(thread: &ThreadData, model: &Model, frame: &mut Frame, area: Rect) {
    let comment = thread
        .comments
        .get(model.data.selected_comment as usize)
        .unwrap();
    let text = comment.get_lines();
    let parag = Paragraph::new(text)
        .block(
            Block::new()
                .title(thread.title.clone())
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red)),
        )
        .style(Style::new().white())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .scroll((model.viewer_scroll, 0));
    frame.render_widget(parag, area);
}

fn render_comment_list(thread: &ThreadData, model: &Model, frame: &mut Frame, area: Rect) {
    let rows = thread
        .comments
        .iter()
        .map(|x| Row::new(vec![x.author.as_str(), x.date.as_str()]));
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
    state.select(Some(model.data.selected_comment.into()));
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
    state.select(Some(model.selected_thread.into()));
    frame.render_stateful_widget(threads_list, area, &mut state);
}
