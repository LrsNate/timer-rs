use crate::timers::stopwatch::Stopwatch;
use crate::timers::timekeeper::Timekeeper;

pub struct AppState {
    pub selected_tab: usize,
    stopwatch: Stopwatch,
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

    pub fn timekeeper(&self) -> &dyn Timekeeper {
        &self.stopwatch
    }

    pub fn timekeeper_mut(&mut self) -> &mut dyn Timekeeper {
        &mut self.stopwatch
    }
}
