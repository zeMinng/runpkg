use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::constants::app::{APP_NAME, APP_VERSION};
use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(format!(" {APP_NAME} "), theme::title()),
        Span::raw("  Interactive package.json Workspace Manager"),
        Span::styled(format!("  v{APP_VERSION}"), theme::dim()),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(title, area);
}
