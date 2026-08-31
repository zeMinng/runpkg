use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::constants::app::{APP_NAME, APP_VERSION};
use crate::tui::theme;

/// Render the application header.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(format!(" {APP_NAME} "), theme::title()),
        Span::raw(format!("  {}", app.project_name())),
        Span::styled(format!("  v{}", app.project_version()), theme::dim()),
        Span::styled(format!("  runpkg {APP_VERSION}"), theme::dim()),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(title, area);
}
