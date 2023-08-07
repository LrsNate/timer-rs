use crate::state::AppState;
use crossterm::event::KeyCode;

pub fn handle_key_event(keycode: KeyCode, state: &mut AppState) {
    match keycode {
        KeyCode::Char('1') => state.switch_tab(0),
        KeyCode::Char('2') => state.switch_tab(1),
        KeyCode::Char('r') => state.stopwatch.reset(),
        KeyCode::Char(' ') => state.stopwatch.toggle_pause(),
        _ => (),
    }
}
