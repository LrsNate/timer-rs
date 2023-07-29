use std::collections::HashMap;

use tui::{style::Color, widgets::canvas::Rectangle};

pub fn get_char_shape(x_offset: f64, y_offset: f64, c: char) -> Vec<Rectangle> {
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

pub fn get_small_char_shape(x_offset: f64, y_offset: f64, c: char) -> Vec<Rectangle> {
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

pub fn get_separator_shape(x_offset: f64, y_offset: f64) -> Vec<Rectangle> {
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
