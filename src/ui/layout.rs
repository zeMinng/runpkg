use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crate::ui::App;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Menu Body
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    crate::ui::components::header::render_header(f, app, chunks[0]);
    crate::ui::components::menu::render_menu(f, app, chunks[1]);
    crate::ui::components::footer::render_footer(f, app, chunks[2]);
}