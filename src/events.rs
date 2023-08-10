use crossterm::event::KeyCode;

use crate::state::AppState;

pub fn handle_key_event(keycode: KeyCode, state: &mut AppState) {
    match keycode {
        KeyCode::Char('1') => state.switch_tab(0),
        KeyCode::Char('2') => state.switch_tab(1),
        KeyCode::Char('3') => state.switch_tab(2),
        KeyCode::Char('a') => state.timekeeper_mut().advance(),
        KeyCode::Char('r') => state.timekeeper_mut().reset(),
        KeyCode::Char('b') => state.play_sound(),
        KeyCode::Char(' ') => state.timekeeper_mut().toggle_pause(),
        _ => (),
    }
}
