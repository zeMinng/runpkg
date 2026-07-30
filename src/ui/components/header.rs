use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::ui::App;

pub fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let title = format!(" 📦 {} (v{}) ", app.env.project_name, app.env.version);
    let info_line = Line::from(vec![
        Span::styled(" 🟢 Node.js ", Style::default().fg(Color::Green)),
        Span::raw(&app.env.node_version),
        Span::raw("  │  "),
        Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
        Span::raw(&app.env.pm_type),
        Span::raw("  │  "),
        Span::styled("🌿 ", Style::default().fg(Color::Cyan)),
        Span::raw(&app.env.git_branch),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(info_line).block(block);
    f.render_widget(paragraph, area);
}