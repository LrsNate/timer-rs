use crate::timer::Stopwatch;

pub struct AppState {
    pub stopwatch: Stopwatch,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            stopwatch: Stopwatch::new(),
        }
    }
}
