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
        ('0', 119),
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

fn get_small_char_shape(x_offset: f64, y_offset: f64, c: char) -> Vec<Rectangle> {
    let char_codes = HashMap::from([
        ('0', 119),
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
            y: y_offset + 8.0,
            width: 3.0,
            height: 1.0,
            color: Color::DarkGray,
        });
    }
    if code >> 5 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset,
            y: y_offset + 5.0,
            width: 1.0,
            height: 3.0,
            color: Color::DarkGray,
        });
    }
    if code >> 4 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 4.0,
            y: y_offset + 5.0,
            width: 1.0,
            height: 3.0,
            color: Color::DarkGray,
        });
    }
    if code >> 3 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 1.0,
            y: y_offset + 4.0,
            width: 3.0,
            height: 1.0,
            color: Color::DarkGray,
        });
    }
    if code >> 2 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset,
            y: y_offset + 1.0,
            width: 1.0,
            height: 3.0,
            color: Color::DarkGray,
        });
    }
    if code >> 1 & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 4.0,
            y: y_offset + 1.0,
            width: 1.0,
            height: 3.0,
            color: Color::DarkGray,
        });
    }
    if code & 1 == 1 {
        result.push(Rectangle {
            x: x_offset + 1.0,
            y: y_offset,
            width: 3.0,
            height: 1.0,
            color: Color::DarkGray,
        });
    }

    result
}

fn get_separator_shape(x_offset: f64, y_offset: f64) -> Vec<Rectangle> {
    vec![
        Rectangle {
            x: x_offset,
            y: y_offset + 3.0,
            width: 1.0,
            height: 1.0,
            color: Color::White,
        },
        Rectangle {
            x: x_offset,
            y: y_offset + 7.0,
            width: 1.0,
            height: 1.0,
            color: Color::White,
        },
    ]
}

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
