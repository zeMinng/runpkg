use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::{App, Focus};
use crate::tui::components::{output, script_list};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let focused = app.focus == Focus::Content;
    let scripts = app.project().map(|project| &project.scripts);
    script_list::render(frame, chunks[0], scripts, app.script_cursor, focused);
    output::render(frame, chunks[1], &app.output);
}
