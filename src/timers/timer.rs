use std::ops::Add;
use std::time::{Duration, Instant};

use chrono::Local;

use crate::timers::timekeeper::{Timekeeper, TimingEvent};

const DEFAULT_DURATION: Duration = Duration::from_secs(60);

pub struct Timer {
    target: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            target: Instant::now() + DEFAULT_DURATION,
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(DEFAULT_DURATION),
        }
    }
}

impl Timekeeper for Timer {
    fn tick(&mut self) -> Option<TimingEvent> {
        self.previous_tick = self.current_tick;
        self.current_tick = Instant::now();
        if self.target.duration_since(self.current_tick) == Duration::ZERO
            && self.target.duration_since(self.previous_tick) != Duration::ZERO
        {
            Some(TimingEvent::TimeUp)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.target = Instant::now() + DEFAULT_DURATION;
        self.paused_duration = Some(DEFAULT_DURATION);
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

    fn extra_display(&self) -> String {
        if self.paused_duration.is_some() {
            String::new()
        } else {
            let now = Local::now();
            let remaining_time = chrono::Duration::from_std(self.time()).unwrap();
            let target = now.add(remaining_time);
            format!("Target time: {}", target.format("%H:%M:%S"))
        }
    }
}
