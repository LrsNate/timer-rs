use std::{collections::HashMap, io::Stdout};

use tui::{
    backend::CrosstermBackend,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders,
    },
    Frame,
};

fn get_char_shape(x_offset: f64, y_offset: f64, c: char) -> Vec<Rectangle> {
    let char_codes = HashMap::from([
        ('1', 18),
        ('2', 93),
        ('3', 91),
        ('4', 58),
        ('5', 107),
        ('6', 111),
        ('7', 82),
        ('8', 127),
        ('9', 123),
    ]);
    let code = char_codes[&c];
    let mut result = Vec::new();
    if code >> 6 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 1.0,
            y: y_offset + 10.0,
            width: 4.0,
            height: 1.0,
            color: Color::White,
        });
    }
    if code >> 5 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset,
            y: y_offset + 6.0,
            width: 1.0,
            height: 4.0,
            color: Color::White,
        });
    }
    if code >> 4 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 5.0,
            y: y_offset + 6.0,
            width: 1.0,
            height: 4.0,
            color: Color::White,
        });
    }
    if code >> 3 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 1.0,
            y: y_offset + 5.0,
            width: 4.0,
            height: 1.0,
            color: Color::White,
        });
    }
    if code >> 2 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset,
            y: y_offset + 1.0,
            width: 1.0,
            height: 4.0,
            color: Color::White,
        });
    }
    if code >> 1 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 5.0,
            y: y_offset + 1.0,
            width: 1.0,
            height: 4.0,
            color: Color::White,
        });
    }
    if code & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 1.0,
            y: y_offset,
            width: 4.0,
            height: 1.0,
            color: Color::White,
        });
    }

    result
}

pub fn draw_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>) {
    let size = f.size();
    let canvas = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .marker(Marker::Braille)
        .x_bounds([0.0, 31.0])
        .y_bounds([0.0, 13.0])
        .paint(|ctx| {
            ctx.layer();
            for line in get_char_shape(1.0, 1.0, '2') {
                ctx.draw(&line);
            }
            for line in get_char_shape(8.0, 1.0, '3') {
                ctx.draw(&line);
            }

            ctx.draw(&Rectangle {
                x: 15.0,
                y: 4.0,
                width: 1.0,
                height: 1.0,
                color: Color::White,
            });

            ctx.draw(&Rectangle {
                x: 15.0,
                y: 8.0,
                width: 1.0,
                height: 1.0,
                color: Color::White,
            });

            for line in get_char_shape(17.0, 1.0, '4') {
                ctx.draw(&line);
            }
            for line in get_char_shape(24.0, 1.0, '5') {
                ctx.draw(&line);
            }
        });
    f.render_widget(canvas, size);
}
