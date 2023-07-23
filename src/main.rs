mod draw;
mod term;

use std::io::Error;

use crossterm::event::{read, Event, KeyCode};
use draw::draw_block;
use term::{close_terminal, init_terminal};

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;

    term.draw(draw_block)?;
    loop {
        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => break,
            _ => (),
        };
    }

    // restore terminal
    close_terminal(&mut term)?;
    Ok(())
}
