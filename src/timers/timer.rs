use std::time::{Duration, Instant};

use crate::timers::timekeeper::Timekeeper;

pub struct Timer {
    target: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl Timer {
    pub fn new() -> Timer {
        let default_duration = Duration::from_secs(25 * 60);
        Timer {
            target: Instant::now() + default_duration,
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(default_duration),
        }
    }
}

impl Timekeeper for Timer {
    fn tick(&mut self) {
        self.previous_tick = self.current_tick;
        self.current_tick = Instant::now();
    }

    fn reset(&mut self) {
        self.target = Instant::now() + Duration::from_secs(25 * 60);
        self.paused_duration = Some(Duration::from_secs(25 * 60));
        self.tick();
    }

    fn toggle_pause(&mut self) {
        self.tick();
        if self.paused_duration.is_none() {
            self.paused_duration = Some(self.time());
        } else {
            self.target = self.current_tick + self.paused_duration.unwrap();
            self.paused_duration = None;
        }
    }

    fn time(&self) -> Duration {
        self.paused_duration
            .unwrap_or(self.target.duration_since(self.current_tick))
    }

    fn latency(&self) -> Duration {
        self.current_tick.duration_since(self.previous_tick)
    }
}
