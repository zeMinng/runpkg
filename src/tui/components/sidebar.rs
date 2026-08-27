use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{Focus, Screen};
use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect, screen: Screen, focus: Focus) {
    let focused = focus == Focus::Sidebar;

    let lines: Vec<Line> = Screen::ALL
        .iter()
        .copied()
        .map(|item| {
            if item == screen {
                if focused {
                    Line::from(Span::styled(format!("▶ {}", item.title()), theme::accent_bold()))
                } else {
                    Line::from(Span::styled(format!("▸ {}", item.title()), theme::dim()))
                }
            } else {
                Line::from(format!("  {}", item.title()))
            }
        })
        .collect();

    let border = if focused {
        theme::active_border()
    } else {
        theme::muted_border()
    };

    let sidebar = Paragraph::new(lines).block(
        Block::default()
            .title(" Menu ")
            .borders(Borders::ALL)
            .border_style(border),
    );

    frame.render_widget(sidebar, area);
}
