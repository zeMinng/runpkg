pub mod components;
pub mod event;
pub mod screens;
pub mod terminal;
pub mod theme;

use std::time::Duration;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{Action, App};
use crate::constants::tui::FRAME_RATE;

use event::{next_event, Event};
use terminal::TerminalGuard;

/// Run the TUI event loop until the app signals it should quit.
pub fn run(app: &mut App) -> anyhow::Result<()> {
    let mut terminal = TerminalGuard::new()?;

    loop {
        app.start_pending_script();
        app.drain_script_output();

        terminal.terminal_mut().draw(|frame| render(frame, app))?;

        if let Event::Key(key) = next_event(Duration::from_millis(FRAME_RATE))?
            && key.kind == KeyEventKind::Press
            && let Some(action) = map_key(key.code)
        {
            app.update(action);
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Orchestrate the chrome (header/sidebar/status bar) and dispatch the content.
fn render(frame: &mut Frame, app: &App) {
    let area = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    components::header::render(frame, layout[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(1)])
        .split(layout[1]);

    components::sidebar::render(frame, body[0], app.screen, app.focus);
    screens::render_content(frame, app, body[1]);

    components::status_bar::render(frame, layout[2], app.screen, app.focus);
}

/// Map a key press to an `Action`.
fn map_key(code: KeyCode) -> Option<Action> {
    match code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Esc => Some(Action::Back),
        KeyCode::Char('r') => Some(Action::Refresh),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::NavigateUp),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::NavigateDown),
        KeyCode::Left | KeyCode::Char('h') => Some(Action::NavigateLeft),
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => Some(Action::NavigateRight),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Char('1') => Some(Action::OpenDashboard),
        KeyCode::Char('2') => Some(Action::OpenScripts),
        KeyCode::Char('3') => Some(Action::OpenDependencies),
        KeyCode::Char('4') => Some(Action::OpenDoctor),
        _ => None,
    }
}
