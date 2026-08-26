use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{Focus, Screen};

pub fn render(frame: &mut Frame, area: Rect, screen: Screen, focus: Focus) {
    let hints: Vec<(&str, &str)> = match (screen, focus) {
        (Screen::Scripts, Focus::Content) => vec![
            ("↑↓", "Select"),
            ("Enter", "Run"),
            ("←/Esc", "Menu"),
            ("r", "Refresh"),
            ("q", "Quit"),
        ],
        (Screen::Dependencies, Focus::Content) => vec![
            ("↑↓", "Select"),
            ("←/Esc", "Menu"),
            ("r", "Refresh"),
            ("q", "Quit"),
        ],
        (_, Focus::Content) => vec![("←/Esc", "Menu"), ("q", "Quit")],
        (_, Focus::Sidebar) => vec![
            ("↑↓", "Menu"),
            ("→/Tab/Enter", "Open"),
            ("1-4", "Jump"),
            ("q", "Quit"),
        ],
    };

    let spans: Vec<Span> = hints
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(format!(" {key} "), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{desc}   ")),
            ]
        })
        .collect();

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
