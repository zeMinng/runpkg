use indexmap::IndexMap;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::tui::theme;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    scripts: Option<&IndexMap<String, String>>,
    cursor: usize,
    focused: bool,
) {
    let has_items = scripts.map(|s| !s.is_empty()).unwrap_or(false);

    let items: Vec<ListItem> = match scripts {
        Some(scripts) => scripts
            .iter()
            .map(|(name, command)| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!(" {name}")),
                    Span::raw("  "),
                    Span::styled(command.clone(), theme::dim()),
                ]))
            })
            .collect(),
        None => vec![ListItem::new(Line::from(Span::styled(
            "No package.json loaded",
            theme::dim(),
        )))],
    };

    let border = if focused {
        theme::active_border()
    } else {
        theme::muted_border()
    };
    let highlight = if focused {
        theme::selected()
    } else {
        theme::muted_selected()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Scripts ")
                .borders(Borders::ALL)
                .border_style(border),
        )
        .highlight_style(highlight);

    let mut state = ListState::default();
    if has_items {
        state.select(Some(cursor));
    }

    frame.render_stateful_widget(list, area, &mut state);
}
