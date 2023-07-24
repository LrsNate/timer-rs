mod draw;
mod term;

use std::{
    io::Error,
    time::{Duration, Instant},
};

use crossterm::event::{poll, read, Event, KeyCode};
use draw::draw_block;
use term::{close_terminal, init_terminal};

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;
    let start_time = Instant::now();

    loop {
        let elapsed = start_time.elapsed();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() - minutes * 60;
        let tenths = (elapsed.as_millis() - (elapsed.as_secs() as u128) * 1000) / 100;
        term.draw(|f| draw_block(f, &format!("{:02}{:02}{:01}", minutes, seconds, tenths)))?;
        if !poll(Duration::from_secs(0))? {
            continue;
        }
        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => break,
            _ => (),
        };
    }

    // restore terminal
    close_terminal(&mut term)?;
    Ok(())
}
