use std::{io::Error, time::Duration};

use crossterm::event::{Event, KeyCode, poll, read};

use term::{close_terminal, init_terminal};

use crate::draw::draw_layout;
use crate::state::AppState;

mod draw;
mod shapes;
mod term;
mod timer;
mod state;

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
