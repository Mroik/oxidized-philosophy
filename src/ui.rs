use ratatui::{Frame, layout::{Layout, Direction, Constraint, Rect}, widgets::{List, Block, Borders, ListState}, style::{Style, Color}};

use crate::model::Model;

fn generate_layout(frame: &Frame) -> (Rect, Rect, Rect) {
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(frame.size());
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(root[1]);
    return (root[0], right[0], right[1]);
}

pub fn view(model: &Model, frame: &mut Frame) {
    let (overview, comments, viewer) = generate_layout(frame);

    let list = List::new(model.overview.iter().map(|item| item.title.clone()))
        .block(
            Block::default()
            .title("Overview")
            .borders(Borders::ALL))
        .style(
            Style::default()
            .fg(Color::White))
        .highlight_style(
            Style::default()
            .bg(Color::LightBlue))
        .highlight_symbol(">>");
    let mut state = ListState::default();
    state.select(Some(model.selected.into()));

    frame.render_stateful_widget(list, overview, &mut state);
    // TODO
}
