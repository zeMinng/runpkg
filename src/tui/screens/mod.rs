pub mod dashboard;
pub mod dependencies;
pub mod doctor;
pub mod scripts;

use ratatui::{layout::Rect, Frame};

use crate::app::{App, Screen};

/// Render the content region for the currently active screen.
pub fn render_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.screen {
        Screen::Dashboard => dashboard::render(frame, app, area),
        Screen::Scripts => scripts::render(frame, app, area),
        Screen::Dependencies => dependencies::render(frame, app, area),
        Screen::Doctor => doctor::render(frame, app, area),
    }
}
