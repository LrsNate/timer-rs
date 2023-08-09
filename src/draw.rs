use std::io::Stdout;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::{backend::CrosstermBackend, Frame};

use crate::state::AppState;
use crate::widgets::{status, timing};

pub fn draw_layout(f: &mut Frame<'_, CrosstermBackend<Stdout>>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_tabs_block(f, chunks[0], state);
    timing::draw_timer_block(f, chunks[1], state);
    status::draw_status_block(f, chunks[2], state);
}

fn draw_tabs_block(f: &mut Frame<'_, CrosstermBackend<Stdout>>, area: Rect, state: &AppState) {
    let titles = ["(1) Stopwatch", "(2) Timer", "(3) Pomodoro"]
        .iter()
        .cloned()
        .map(Line::from)
        .collect();
    let tabs = Tabs::new(titles)
        .select(state.selected_tab)
        .block(Block::default().borders(Borders::TOP.union(Borders::BOTTOM)))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .divider(Span::from("|"));
    f.render_widget(tabs, area);
}
