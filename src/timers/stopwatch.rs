use std::time::{Duration, Instant};

use crate::timers::timekeeper::Timekeeper;

pub struct Stopwatch {
    start: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start: Instant::now(),
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(Duration::ZERO),
        }
    }

    fn pause(&mut self) {
        self.paused_duration = Some(self.time());
    }

    fn unpause(&mut self) {
        self.start = self.current_tick - self.paused_duration.unwrap();
        self.paused_duration = None;
    }
}

impl Timekeeper for Stopwatch {
    fn tick(&mut self) {
        self.previous_tick = self.current_tick;
        self.current_tick = Instant::now();
    }

    fn reset(&mut self) {
        self.start = Instant::now();
        self.paused_duration = None;
        self.tick();
    }

    fn toggle_pause(&mut self) {
        self.tick();
        if self.paused_duration.is_none() {
            self.pause()
        } else {
            self.unpause()
        }
    }

    fn time(&self) -> Duration {
        self.paused_duration
            .unwrap_or(self.current_tick.duration_since(self.start))
    }

    fn latency(&self) -> Duration {
        self.current_tick.duration_since(self.previous_tick)
    }
}
