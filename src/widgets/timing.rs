use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Marker;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::shapes::{get_char_shape, get_separator_shape, get_small_char_shape};
use crate::state::AppState;

pub fn draw_timer_block(f: &mut Frame, area: Rect, state: &AppState) {
    if state.timekeeper().secondary_display().is_empty() {
        draw_primary_block(f, area, state);
    } else {
        let timing_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(12)].as_ref())
            .split(area);
        draw_primary_block(f, timing_chunks[0], state);
        draw_secondary_block(f, timing_chunks[1], state);
    }
}

fn draw_primary_block(f: &mut Frame, area: Rect, state: &AppState) {
    let s = state.timekeeper().display();
    let chars: Vec<char> = s.chars().collect();
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, 40.0])
        .y_bounds([0.0, 13.0])
        .paint(|ctx| {
            ctx.layer();
            let mut shapes = vec![];
            shapes.append(&mut get_char_shape(1.0, 1.0, chars[0]));
            shapes.append(&mut get_char_shape(9.0, 1.0, chars[1]));
            if ['0', '1', '2', '3', '4'].contains(&chars[4]) {
                shapes.append(&mut get_separator_shape(16.0, 1.0));
            }
            shapes.append(&mut get_char_shape(18.0, 1.0, chars[2]));
            shapes.append(&mut get_char_shape(26.0, 1.0, chars[3]));
            shapes.append(&mut get_small_char_shape(34.0, 1.0, chars[4]));

            for line in shapes {
                ctx.draw(&line);
            }
        });
    f.render_widget(canvas, area);
}

fn draw_secondary_block(f: &mut Frame, area: Rect, state: &AppState) {
    let paragraph = Paragraph::new(state.timekeeper().secondary_display())
        .block(Block::new().title("Laps").borders(Borders::LEFT));
    f.render_widget(paragraph, area);
}
