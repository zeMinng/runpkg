use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::ui::App;

pub fn render_menu(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🚀 runpkg Project Console ")
        .border_style(Style::default().fg(Color::White));

    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, (title, desc))| {
            let content = if i == app.selected_index {
                Line::from(vec![
                    Span::styled(
                        " ❯ ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        *title,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("   ({})", desc),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::raw("   "),
                    Span::raw(*title),
                    Span::styled(
                        format!("   ({})", desc),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}