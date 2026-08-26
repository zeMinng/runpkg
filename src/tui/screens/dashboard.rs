use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let project = app.state.project.as_ref();
    let runtime = app.state.runtime.as_ref();

    let name = project
        .map(|p| format!("{} v{}", p.name, p.version))
        .unwrap_or_else(|| "Not found".to_string());
    let node = runtime
        .and_then(|r| r.node_version.clone())
        .unwrap_or_else(|| "-".to_string());
    let pms = runtime
        .map(|r| {
            if r.available_package_managers.is_empty() {
                "-".to_string()
            } else {
                r.available_package_managers.join(", ")
            }
        })
        .unwrap_or_else(|| "-".to_string());
    let script_count = project.map(|p| p.scripts.len()).unwrap_or(0);
    let dep_count = project
        .map(|p| p.dependencies.len() + p.dev_dependencies.len() + p.peer_dependencies.len())
        .unwrap_or(0);

    let lines = vec![
        Line::from(Span::styled(
            "Project Dashboard",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Project"),
        Line::from(format!("  {name}")),
        Line::from(format!("  Scripts          {script_count}")),
        Line::from(format!("  Dependencies     {dep_count}")),
        Line::from(""),
        Line::from("Runtime"),
        Line::from(format!("  Node             {node}")),
        Line::from(format!("  Package Manager  {pms}")),
    ];

    let content = Paragraph::new(lines).block(
        Block::default().title(" Dashboard ").borders(Borders::ALL),
    );

    frame.render_widget(content, area);
}
