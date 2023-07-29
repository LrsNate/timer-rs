use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    symbols::Marker,
    widgets::{canvas::Canvas, Block, Borders},
    Frame,
};

use crate::shapes::{get_char_shape, get_separator_shape, get_small_char_shape};

pub fn draw_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>, s: &str) {
    let size = f.size();
    let canvas = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .marker(Marker::Braille)
        .x_bounds([0.0, 40.0])
        .y_bounds([0.0, 13.0])
        .paint(|ctx| {
            ctx.layer();
            let mut shapes = vec![];
            shapes.append(&mut get_char_shape(1.0, 1.0, s.chars().nth(0).unwrap()));
            shapes.append(&mut get_char_shape(9.0, 1.0, s.chars().nth(1).unwrap()));
            shapes.append(&mut get_separator_shape(16.0, 1.0));
            shapes.append(&mut get_char_shape(18.0, 1.0, s.chars().nth(2).unwrap()));
            shapes.append(&mut get_char_shape(26.0, 1.0, s.chars().nth(3).unwrap()));
            shapes.append(&mut get_small_char_shape(
                34.0,
                1.0,
                s.chars().nth(4).unwrap(),
            ));

            for line in shapes {
                ctx.draw(&line);
            }
        });
    f.render_widget(canvas, size);
}
