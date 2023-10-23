use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::Frame;

use crate::state::AppState;

pub fn draw_tabs_block(f: &mut Frame, area: Rect, state: &AppState) {
    let titles = ["Stopwatch", "Timer", "Pomodoro"]
        .iter()
        .cloned()
        .map(Line::from)
        .collect();
    let tabs = Tabs::new(titles)
        .select(state.selected_tab)
        .block(Block::default().borders(Borders::TOP.union(Borders::BOTTOM)))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider(Span::from("|"));
    f.render_widget(tabs, area);
}
