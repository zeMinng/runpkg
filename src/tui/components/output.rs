use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::OutputState;
use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect, output: &OutputState) {
    let title = match (&output.running, output.exit_code) {
        (Some(name), _) => format!(" Output · running `{name}` "),
        (_, Some(code)) => format!(" Output · exited ({code}) "),
        _ => " Output ".to_string(),
    };

    let border = if output.exit_code == Some(0) {
        theme::ok()
    } else if output.exit_code.is_some() {
        theme::error()
    } else {
        theme::accent()
    };

    let visible = area.height.saturating_sub(2) as usize;
    let tail = if output.lines.len() > visible {
        &output.lines[output.lines.len() - visible..]
    } else {
        &output.lines[..]
    };

    let body: Vec<Line> = if tail.is_empty() {
        vec![Line::from(Span::styled(
            "Run a script to see its output here.",
            theme::dim(),
        ))]
    } else {
        tail.iter().map(|l| Line::from(l.as_str())).collect()
    };

    let content = Paragraph::new(body).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border),
    );

    frame.render_widget(content, area);
}
