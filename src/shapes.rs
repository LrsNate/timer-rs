use phf::{phf_map, Map};
use ratatui::{style::Color, widgets::canvas::Rectangle};

const CHAR_CODES: Map<char, u8> = phf_map! {
    '0' => 119,
    '1' => 18,
    '2' => 93,
    '3' => 91,
    '4' => 58,
    '5' => 107,
    '6' => 111,
    '7' => 82,
    '8' => 127,
    '9' => 123,
};

fn horizontal_bar(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x,
        y,
        width: 4.0,
        height: 1.0,
        color: Color::White,
    }
}

fn vertical_bar(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x,
        y,
        width: 1.0,
        height: 4.0,
        color: Color::White,
    }
}

pub fn get_char_shape(x_offset: f64, y_offset: f64, c: char) -> Vec<Rectangle> {
    let code = CHAR_CODES[&c];
    let mut result = Vec::new();
    if code >> 6 & 1 == 1 {
        result.push(horizontal_bar(x_offset + 1.0, y_offset + 10.0));
    }
    if code >> 5 & 1 == 1 {
        result.push(vertical_bar(x_offset, y_offset + 6.0));
    }
    if code >> 4 & 1 == 1 {
        result.push(vertical_bar(x_offset + 5.0, y_offset + 6.0));
    }
    if code >> 3 & 1 == 1 {
        result.push(horizontal_bar(x_offset + 1.0, y_offset + 5.0));
    }
    if code >> 2 & 1 == 1 {
        result.push(vertical_bar(x_offset, y_offset + 1.0));
    }
    if code >> 1 & 1 == 1 {
        result.push(vertical_bar(x_offset + 5.0, y_offset + 1.0));
    }
    if code & 1 == 1 {
        result.push(horizontal_bar(x_offset + 1.0, y_offset));
    }

    result
}

fn small_horizontal_bar(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x,
        y,
        width: 3.0,
        height: 1.0,
        color: Color::DarkGray,
    }
}

fn small_vertical_bar(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x,
        y,
        width: 1.0,
        height: 3.0,
        color: Color::DarkGray,
    }
}

pub fn get_small_char_shape(x_offset: f64, y_offset: f64, c: char) -> Vec<Rectangle> {
    let code = CHAR_CODES[&c];
    let mut result = Vec::new();
    if code >> 6 & 1 == 1 {
        result.push(small_horizontal_bar(x_offset + 1.0, y_offset + 8.0));
    }
    if code >> 5 & 1 == 1 {
        result.push(small_vertical_bar(x_offset, y_offset + 5.0));
    }
    if code >> 4 & 1 == 1 {
        result.push(small_vertical_bar(x_offset + 4.0, y_offset + 5.0));
    }
    if code >> 3 & 1 == 1 {
        result.push(small_horizontal_bar(x_offset + 1.0, y_offset + 4.0));
    }
    if code >> 2 & 1 == 1 {
        result.push(small_vertical_bar(x_offset, y_offset + 1.0));
    }
    if code >> 1 & 1 == 1 {
        result.push(small_vertical_bar(x_offset + 4.0, y_offset + 1.0));
    }
    if code & 1 == 1 {
        result.push(small_horizontal_bar(x_offset + 1.0, y_offset));
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
