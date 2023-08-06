use std::{io::Error, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use term::{close_terminal, init_terminal};

use crate::draw::draw_layout;
use crate::state::AppState;

mod draw;
mod shapes;
mod state;
mod term;
mod timer;

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;
    let mut state = AppState::new();

    loop {
        state.timer.tick();
        term.draw(|f| draw_layout(f, &state))?;
        if !poll(Duration::from_secs(0))? {
            continue;
        }
        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => break,
            Event::Key(event) if event.code == KeyCode::Char('r') => state.timer.reset(),
            _ => (),
        };
    }

    // restore terminal
    close_terminal(&mut term)?;
    Ok(())
}
