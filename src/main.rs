mod draw;
mod shapes;
mod term;
mod timer;

use std::{io::Error, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};
use draw::draw_block;
use term::{close_terminal, init_terminal};
use timer::Timer;

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;
    let mut timer = Timer::new();

    loop {
        timer.tick();
        term.draw(|f| draw_block(f, &format!("{}", timer)))?;
        if !poll(Duration::from_secs(0))? {
            continue;
        }
        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => break,
            Event::Key(event) if event.code == KeyCode::Char('r') => timer.reset(),
            _ => (),
        };
    }

    // restore terminal
    close_terminal(&mut term)?;
    Ok(())
}
