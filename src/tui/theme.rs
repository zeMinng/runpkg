//! Centralized TUI styles and palette. (集中管理的 TUI 样式与调色板)

use ratatui::style::{Modifier, Style};

pub mod palette {
    use ratatui::style::Color;

    pub const ACCENT: Color = Color::Cyan;
    pub const MUTED: Color = Color::DarkGray;
    pub const OK: Color = Color::Green;
    pub const ERROR: Color = Color::Red;
    pub const BG: Color = Color::Black;
}

/// Row highlight for lists. (列表选中行高亮)
pub fn selected() -> Style {
    Style::default()
        .fg(palette::BG)
        .bg(palette::ACCENT)
        .add_modifier(Modifier::BOLD)
}

/// Bold accent text (sidebar cursor / section headers).
pub fn accent_bold() -> Style {
    Style::default().fg(palette::ACCENT).add_modifier(Modifier::BOLD)
}

/// Plain accent text (active screen marker).
pub fn accent() -> Style {
    Style::default().fg(palette::ACCENT)
}

/// Title chip (header brand).
pub fn title() -> Style {
    Style::default()
        .fg(palette::BG)
        .bg(palette::ACCENT)
        .add_modifier(Modifier::BOLD)
}

/// Dimmed secondary text.
pub fn dim() -> Style {
    Style::default().fg(palette::MUTED)
}

/// Success emphasis.
pub fn ok() -> Style {
    Style::default().fg(palette::OK)
}

/// Failure emphasis.
pub fn error() -> Style {
    Style::default().fg(palette::ERROR)
}

/// Border for the pane that owns keyboard focus. (拥有焦点面板的边框)
pub fn active_border() -> Style {
    Style::default().fg(palette::ACCENT)
}

/// Border for panes without keyboard focus. (无焦点面板的边框)
pub fn muted_border() -> Style {
    Style::default().fg(palette::MUTED)
}

/// Row highlight when the pane is not focused. (无焦点面板的选中行)
pub fn muted_selected() -> Style {
    Style::default()
        .fg(palette::MUTED)
        .add_modifier(Modifier::BOLD)
}
