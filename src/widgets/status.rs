use std::ops::Add;

use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::state::AppState;

pub fn draw_status_block(f: &mut Frame, area: Rect, state: &AppState) {
    let latency = state.timekeeper().latency();
    let mut text = format!("Latency: {} ms", latency.as_millis());
    let extra_display = state.timekeeper().extra_display();
    if !extra_display.is_empty() {
        text = text.add(format!(" - {}", extra_display).as_str());
    }
    let paragraph = Paragraph::new(Span::raw(text))
        .style(Style::default().fg(Color::Black).bg(Color::LightBlue));
    f.render_widget(paragraph, area);
}
