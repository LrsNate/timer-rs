use std::io::Stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Span, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::AppState;

pub fn draw_status_block(
    f: &mut Frame<'_, CrosstermBackend<Stdout>>,
    area: Rect,
    state: &AppState,
) {
    let latency = state.timekeeper().latency();
    let text = Span::raw(format!("Latency: {} ms", latency.as_millis()));
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Black).bg(Color::Blue));
    f.render_widget(paragraph, area);
}
