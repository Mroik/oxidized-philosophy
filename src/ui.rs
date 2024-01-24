use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListState, Paragraph, Wrap},
    Frame,
};

use crate::{model::Model, thread::ThreadComment};

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

    let threads_list = List::new(model.overview.iter().map(|item| item.title.clone()))
        .block(Block::default().title("Overview").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bg(Color::LightBlue))
        .highlight_symbol(">>");
    let mut state = ListState::default();
    state.select(Some(model.selected_thread.into()));
    frame.render_stateful_widget(threads_list, overview, &mut state);

    let thread = model.data.data.get(model.selected_thread as usize).unwrap();
    let comment_list = List::new(thread.comments.iter().map(format_comment_list_row))
        .block(Block::default().title("Comments").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bg(Color::LightBlue))
        .highlight_symbol(">>");
    let mut state = ListState::default();
    state.select(Some(model.data.selected_comment.into()));
    frame.render_stateful_widget(comment_list, comments, &mut state);

    let comment = thread
        .comments
        .get(model.data.selected_comment as usize)
        .unwrap();
    let text = Line::from(comment.get_text());
    let parag = Paragraph::new(text)
        .block(
            Block::new()
                .title(thread.title.clone())
                .borders(Borders::ALL),
        )
        .style(Style::new().white())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((model.viewer_scroll, 0));
    frame.render_widget(parag, viewer);
}

fn format_comment_list_row(item: &ThreadComment) -> String {
    let mut ris = item.author.clone();
    ris.push_str("                                       ");
    ris.push_str(item.date.as_str());
    return ris;
}
