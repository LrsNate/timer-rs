mod draw;
mod shapes;
mod term;
mod timer;
mod state;

use std::{io::Error, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};
use draw::draw_block;
use term::{close_terminal, init_terminal};
use crate::state::AppState;

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;
    let mut state = AppState::new();

    loop {
        state.timer.tick();
        term.draw(|f| draw_block(f, &state))?;
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
