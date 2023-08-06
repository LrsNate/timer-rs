use std::io::Stdout;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::{
    backend::CrosstermBackend,
    symbols::Marker,
    widgets::{canvas::Canvas, Block, Borders},
    Frame,
};

use crate::shapes::{get_char_shape, get_separator_shape, get_small_char_shape};
use crate::state::AppState;

pub fn draw_layout(f: &mut Frame<'_, CrosstermBackend<Stdout>>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());
    draw_block(f, chunks[0], state);
    let text = Span::raw("Some text here");
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, chunks[1]);
}

pub fn draw_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect, state: &AppState) {
    let s = format!("{}", state.timer);
    let chars: Vec<char> = s.chars().collect();
    let canvas = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .marker(Marker::Braille)
        .x_bounds([0.0, 40.0])
        .y_bounds([0.0, 13.0])
        .paint(|ctx| {
            ctx.layer();
            let mut shapes = vec![];
            shapes.append(&mut get_char_shape(1.0, 1.0, chars[0]));
            shapes.append(&mut get_char_shape(9.0, 1.0, chars[1]));
            shapes.append(&mut get_separator_shape(16.0, 1.0));
            shapes.append(&mut get_char_shape(18.0, 1.0, chars[2]));
            shapes.append(&mut get_char_shape(26.0, 1.0, chars[3]));
            shapes.append(&mut get_small_char_shape(34.0, 1.0, chars[4]));

            for line in shapes {
                ctx.draw(&line);
            }
        });
    f.render_widget(canvas, area);
}
