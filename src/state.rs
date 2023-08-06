use crate::timer::Timer;

pub struct AppState {
    pub timer: Timer,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            timer: Timer::new(),
        }
    }
}
