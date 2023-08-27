use std::io::Stdout;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{backend::CrosstermBackend, Frame};

use crate::state::AppState;
use crate::widgets::{status, tabs, timing};

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
    tabs::draw_tabs_block(f, chunks[0], state);
    timing::draw_timer_block(f, chunks[1], state);
    status::draw_status_block(f, chunks[2], state);
}
