use std::path::Path;

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::constants::paths::LOCK_FILES;
use crate::constants::ui::Icons;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        "Project Doctor",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    let project = app.state.project.as_ref();
    let runtime = app.state.runtime.as_ref();

    check(&mut lines, "package.json loaded", project.is_some());
    check(
        &mut lines,
        "project name",
        project.map(|p| !p.name.is_empty() && p.name != "unknown").unwrap_or(false),
    );
    check(
        &mut lines,
        "project version",
        project.map(|p| !p.version.is_empty()).unwrap_or(false),
    );
    check(
        &mut lines,
        "packageManager declared",
        project.map(|p| p.package_manager.is_some()).unwrap_or(false),
    );
    check(
        &mut lines,
        "Node.js detected",
        runtime.and_then(|r| r.node_version.clone()).is_some(),
    );
    check(
        &mut lines,
        "package manager available",
        runtime.map(|r| !r.available_package_managers.is_empty()).unwrap_or(false),
    );

    match detect_lock_file(&app.project_path) {
        Some(lock) => {
            check(&mut lines, "lock file present", true);
            lines.push(Line::from(Span::styled(format!("    → {lock}"), theme::dim())));
        }
        None => check(&mut lines, "lock file present", false),
    }

    let content = Paragraph::new(lines).block(
        Block::default().title(" Doctor ").borders(Borders::ALL),
    );

    frame.render_widget(content, area);
}

fn check(lines: &mut Vec<Line>, label: &str, ok: bool) {
    let (icon, style) = if ok {
        (Icons::SUCCESS, theme::ok())
    } else {
        (Icons::ERROR, theme::error())
    };
    lines.push(Line::from(vec![
        Span::styled(format!(" {icon} "), style),
        Span::raw(label.to_owned()),
    ]));
}

fn detect_lock_file(project_path: &Path) -> Option<&'static str> {
    LOCK_FILES
        .iter()
        .find(|&&name| project_path.join(name).exists())
        .copied()
}
