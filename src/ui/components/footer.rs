use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span}, // 👈 补充了 Span 的导入
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::ui::App;

pub fn render_footer(f: &mut Frame, _app: &App, area: Rect) {
    let info = Line::from(vec![
        Span::styled(" [↑/↓/j/k]", Style::default().fg(Color::Cyan)),
        Span::raw(" 选择  "),
        Span::styled("[Enter]", Style::default().fg(Color::Cyan)),
        Span::raw(" 确认  "),
        Span::styled("[1-5]", Style::default().fg(Color::Cyan)),
        Span::raw(" 数字直达  "),
        Span::styled("[q/Esc]", Style::default().fg(Color::Red)),
        Span::raw(" 退出"),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(info).block(block);
    f.render_widget(paragraph, area);
}