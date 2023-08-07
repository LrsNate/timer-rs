use std::{io::Error, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use term::{close_terminal, init_terminal};

use crate::draw::draw_layout;
use crate::events::handle_key_event;
use crate::state::AppState;

mod draw;
mod events;
mod shapes;
mod state;
mod term;
mod timers;

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;
    let mut state = AppState::new();

    loop {
        state.timekeeper_mut().tick();
        term.draw(|f| draw_layout(f, &state))?;
        if !poll(Duration::from_secs(0))? {
            continue;
        }
        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => break,
            Event::Key(event) => handle_key_event(event.code, &mut state),
            _ => (),
        };
    }

    // restore terminal
    close_terminal(&mut term)?;
    Ok(())
}
