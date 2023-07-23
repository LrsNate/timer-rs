use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders,
    },
    Frame,
};

fn get_char_shape(x_offset: f64, y_offset: f64) -> Vec<Line> {
    let mut result = Vec::new();
    result.push(Line {
        x1: x_offset,
        y1: y_offset,
        x2: x_offset + 3.0,
        y2: y_offset,
        color: Color::White,
    });
    result.push(Line {
        x1: x_offset,
        y1: y_offset,
        x2: x_offset,
        y2: y_offset + 3.0,
        color: Color::White,
    });
    result.push(Line {
        x1: x_offset,
        y1: y_offset + 3.0,
        x2: x_offset + 3.0,
        y2: y_offset + 3.0,
        color: Color::White,
    });
    result.push(Line {
        x1: x_offset + 3.0,
        y1: y_offset,
        x2: x_offset + 3.0,
        y2: y_offset + 3.0,
        color: Color::White,
    });
    result.push(Line {
        x1: x_offset,
        y1: y_offset + 3.0,
        x2: x_offset,
        y2: y_offset + 6.0,
        color: Color::White,
    });
    result.push(Line {
        x1: x_offset + 3.0,
        y1: y_offset + 3.0,
        x2: x_offset + 3.0,
        y2: y_offset + 6.0,
        color: Color::White,
    });
    result.push(Line {
        x1: x_offset,
        y1: y_offset + 6.0,
        x2: x_offset + 3.0,
        y2: y_offset + 6.0,
        color: Color::White,
    });
    result
}

pub fn draw_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>) {
    let size = f.size();
    let canvas = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .marker(Marker::Braille)
        .x_bounds([0.0, 20.0])
        .y_bounds([0.0, 8.0])
        .paint(|ctx| {
            ctx.layer();
            for line in get_char_shape(1.0, 1.0) {
                ctx.draw(&line);
            }
            for line in get_char_shape(5.0, 1.0) {
                ctx.draw(&line);
            }

            ctx.draw(&Line {
                x1: 9.0,
                y1: 3.0,
                x2: 9.0,
                y2: 3.0,
                color: Color::White,
            });

            ctx.draw(&Line {
                x1: 9.0,
                y1: 5.0,
                x2: 9.0,
                y2: 5.0,
                color: Color::White,
            });
        });
    f.render_widget(canvas, size);
}
