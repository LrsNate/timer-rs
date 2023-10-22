use std::{io::Error, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode};

use term::{close_terminal, init_terminal};

use crate::draw::draw_layout;
use crate::events::handle_key_event;
use crate::settings::get_settings;
use crate::sound::SoundThread;
use crate::state::AppState;
use crate::timers::timekeeper::TimingEvent;

mod draw;
mod events;
mod settings;
mod shapes;
mod sound;
mod state;
mod term;
mod timers;
mod widgets;

fn main() -> Result<(), Error> {
    let mut term = init_terminal()?;
    let settings = get_settings();
    let mut state = AppState::new(settings);
    let mut sound = SoundThread::new();

    loop {
        if let Some(TimingEvent::TimeUp) = state.tick() {
            sound.play_sound();
        }
        term.draw(|f| draw_layout(f, &state))?;
        if !poll(Duration::from_millis(50))? {
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
