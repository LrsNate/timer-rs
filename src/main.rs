mod term;

use std::io::Error;

use crossterm::event::{read, Event, KeyCode};
use term::{close_terminal, init_terminal};
use tui::widgets::{Block, Borders};

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;

    term.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
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
