use crate::timers::stopwatch::Stopwatch;

pub struct AppState {
    pub selected_tab: usize,
    pub stopwatch: Stopwatch,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            selected_tab: 0,
            stopwatch: Stopwatch::new(),
        }
    }

    pub fn switch_tab(&mut self, tab: usize) {
        self.selected_tab = tab;
    }
}
