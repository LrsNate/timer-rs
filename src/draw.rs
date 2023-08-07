use std::io::Stdout;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::DOT;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs};
use ratatui::{backend::CrosstermBackend, symbols::Marker, widgets::canvas::Canvas, Frame};

use crate::shapes::{get_char_shape, get_separator_shape, get_small_char_shape};
use crate::state::AppState;

pub fn draw_layout(f: &mut Frame<'_, CrosstermBackend<Stdout>>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_tabs_block(f, chunks[0], state);
    draw_timer_block(f, chunks[1], state);
    draw_status_block(f, chunks[2], state);
}

fn draw_tabs_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect, state: &AppState) {
    let titles = ["Stopwatch", "Timer"]
        .iter()
        .cloned()
        .map(Line::from)
        .collect();
    let tabs = Tabs::new(titles)
        .select(state.selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider(DOT);
    f.render_widget(tabs, area);
}

fn draw_timer_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect, state: &AppState) {
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

pub fn draw_status_block(
    f: &mut Frame<'_, CrosstermBackend<Stdout>>,
    area: Rect,
    state: &AppState,
) {
    let latency = state.timekeeper().latency();
    let text = Span::raw(format!("Latency: {} ms", latency.as_millis()));
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Black).bg(Color::Blue));
    f.render_widget(paragraph, area);
}
