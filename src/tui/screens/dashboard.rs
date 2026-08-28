use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let node = app.node_version().unwrap_or("-");
    let package_managers = if app.available_package_managers().is_empty() {
        "-".to_owned()
    } else {
        app.available_package_managers().join(", ")
    };

    let lines = vec![
        Line::from(Span::styled(
            "Project Dashboard",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Project"),
        Line::from(format!("  {} v{}", app.project_name(), app.project_version())),
        Line::from(format!("  Scripts          {}", app.script_count())),
        Line::from(format!("  Dependencies     {}", app.dependency_count())),
        Line::from(""),
        Line::from("Runtime"),
        Line::from(format!("  Node             {node}")),
        Line::from(format!("  Package Manager  {package_managers}")),
    ];

    let content = Paragraph::new(lines)
        .block(Block::default().title(" Dashboard ").borders(Borders::ALL));

    frame.render_widget(content, area);
}
