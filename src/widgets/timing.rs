use std::io::Stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::prelude::Marker;
use ratatui::widgets::canvas::Canvas;
use ratatui::Frame;

use crate::shapes::{get_char_shape, get_separator_shape, get_small_char_shape};
use crate::state::AppState;

pub fn draw_timer_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect, state: &AppState) {
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
