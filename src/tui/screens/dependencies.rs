use ratatui::{layout::Rect, Frame};

use crate::app::{App, Focus};
use crate::tui::components::dependency_list;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::Content;
    dependency_list::render(
        frame,
        area,
        app.project(),
        app.dep_cursor,
        focused,
    );
}
