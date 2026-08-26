use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::project::info::{DepEntry, ProjectInfo};
use crate::tui::theme;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    project: Option<&ProjectInfo>,
    cursor: usize,
    focused: bool,
) {
    let mut lines: Vec<Line> = Vec::new();
    let mut flat = 0usize;

    match project {
        Some(project) => {
            push_section(
                &mut lines,
                "Dependencies",
                &project.dependencies,
                cursor,
                &mut flat,
                focused,
            );
            push_section(
                &mut lines,
                "Dev Dependencies",
                &project.dev_dependencies,
                cursor,
                &mut flat,
                focused,
            );
            push_section(
                &mut lines,
                "Peer Dependencies",
                &project.peer_dependencies,
                cursor,
                &mut flat,
                focused,
            );
        }
        None => {
            lines.push(Line::from(Span::styled("No package.json loaded", theme::dim())));
        }
    }

    let border = if focused {
        theme::active_border()
    } else {
        theme::muted_border()
    };

    let content = Paragraph::new(lines).block(
        Block::default()
            .title(" Dependencies ")
            .borders(Borders::ALL)
            .border_style(border),
    );

    frame.render_widget(content, area);
}

fn push_section(
    lines: &mut Vec<Line>,
    title: &str,
    entries: &[DepEntry],
    cursor: usize,
    flat: &mut usize,
    focused: bool,
) {
    lines.push(Line::from(Span::styled(
        format!(" {title} ({})", entries.len()),
        theme::accent_bold(),
    )));

    if entries.is_empty() {
        lines.push(Line::from(Span::styled("   (none)", theme::dim())));
        return;
    }

    for entry in entries {
        let label = format!("{}  {}", entry.name, entry.version);
        if *flat == cursor {
            let (marker, style) = if focused {
                ("▶", theme::accent_bold())
            } else {
                ("▸", theme::dim())
            };
            lines.push(Line::from(Span::styled(format!("{marker} {label}"), style)));
        } else {
            lines.push(Line::from(Span::raw(format!("  {label}"))));
        }
        *flat += 1;
    }
}
