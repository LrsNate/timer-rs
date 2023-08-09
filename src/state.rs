use crate::timers::stopwatch::Stopwatch;
use crate::timers::timekeeper::Timekeeper;
use crate::timers::timer::Timer;

pub struct AppState {
    pub selected_tab: usize,
    stopwatch: Stopwatch,
    timer: Timer,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            selected_tab: 0,
            stopwatch: Stopwatch::new(),
            timer: Timer::new(),
        }
    }

    pub fn switch_tab(&mut self, tab: usize) {
        self.selected_tab = tab;
        self.timekeeper_mut().reset();
    }

    pub fn timekeeper(&self) -> &dyn Timekeeper {
        let timekeepers: [&dyn Timekeeper; 2] = [&self.stopwatch, &self.timer];
        timekeepers[self.selected_tab]
    }

    pub fn timekeeper_mut(&mut self) -> &mut dyn Timekeeper {
        let timekeepers: [&mut dyn Timekeeper; 2] = [&mut self.stopwatch, &mut self.timer];
        timekeepers[self.selected_tab]
    }
}
