use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::constants::app::{APP_NAME, APP_VERSION};
use crate::constants::paths::LOCK_FILES;
use crate::system::pm;
use crate::tui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    let border = Block::default().borders(Borders::BOTTOM).border_style(theme::muted_border());
    frame.render_widget(build_left(app).block(border.clone()), columns[0]);
    frame.render_widget(build_right(app).block(border), columns[1]);
}

/// Left column: brand chip + folder + branch/status + node + run command.
fn build_left(app: &App) -> Paragraph<'static> {
    let project = app.state.project.as_ref();
    let runtime = app.state.runtime.as_ref();

    let folder = app
        .project_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| app.project_path.display().to_string());

    let pm = pm::preferred(project, runtime, &app.project_path);

    let mut groups: Vec<Vec<Span<'static>>> = vec![
        vec![Span::styled(
            format!(" {APP_NAME} v{APP_VERSION} "),
            theme::title(),
        )],
        vec![Span::styled(folder, theme::accent())],
    ];

    if let Some(branch) = &app.git.branch {
        let mut branch_group = vec![Span::styled(format!("⎇ {branch}"), theme::accent_bold())];

        // Change indicators, in `* + -` order (modified / added / deleted).
        if app.git.modified {
            branch_group.push(Span::styled("*", theme::accent_bold()));
        }
        if app.git.added {
            branch_group.push(Span::styled("+", theme::ok()));
        }
        if app.git.deleted {
            branch_group.push(Span::styled("-", theme::error()));
        }
        groups.push(branch_group);
    }

    match runtime.and_then(|r| r.node_version.clone()) {
        Some(version) => groups.push(vec![
            Span::styled("node ", theme::dim()),
            Span::styled(version, theme::accent()),
        ]),
        None => groups.push(vec![Span::styled("node -", theme::dim())]),
    }

    groups.push(vec![
        Span::styled("run ", theme::dim()),
        Span::styled(pm, theme::accent()),
    ]);

    Paragraph::new(Line::from(join_groups(groups)))
}

/// Right column: lightweight project dashboard summary.
fn build_right(app: &App) -> Paragraph<'static> {
    let project = app.state.project.as_ref();

    let scripts = project.map(|p| p.scripts.len()).unwrap_or(0);
    let deps = project
        .map(|p| p.dependencies.len() + p.dev_dependencies.len() + p.peer_dependencies.len())
        .unwrap_or(0);

    let mut groups: Vec<Vec<Span<'static>>> = vec![
        vec![
            Span::styled("Scripts ", theme::dim()),
            Span::styled(scripts.to_string(), theme::accent()),
        ],
        vec![
            Span::styled("Deps ", theme::dim()),
            Span::styled(deps.to_string(), theme::accent()),
        ],
    ];

    if let Some((file, _)) = LOCK_FILES
        .iter()
        .find(|(file, _)| app.project_path.join(file).exists())
    {
        groups.push(vec![Span::styled(format!("lock {file}"), theme::dim())]);
    }

    Paragraph::new(Line::from(join_groups(groups))).alignment(Alignment::Right)
}

/// Join groups of spans with a dimmed ` · ` separator.
fn join_groups(groups: Vec<Vec<Span<'static>>>) -> Vec<Span<'static>> {
    let mut out: Vec<Span<'static>> = Vec::new();
    for (idx, group) in groups.into_iter().enumerate() {
        if idx > 0 {
            out.push(Span::styled(" · ", theme::dim()));
        }
        out.extend(group);
    }
    out
}
