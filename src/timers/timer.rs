use std::ops::Add;
use std::time::{Duration, Instant};

use chrono::Local;
use serde::Deserialize;

use crate::timers::timekeeper::{Timekeeper, TimingEvent};

#[derive(Deserialize)]
pub struct TimerSettings {
    pub duration_seconds: u64,
}

pub struct Timer {
    default_duration: Duration,
    target: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl Timer {
    pub fn new(settings: TimerSettings) -> Timer {
        let default_duration = Duration::from_secs(settings.duration_seconds);
        Timer {
            default_duration,
            target: Instant::now() + default_duration,
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(default_duration),
        }
    }
}

impl Timekeeper for Timer {
    fn tick(&mut self) -> Option<TimingEvent> {
        self.previous_tick = self.current_tick;
        self.current_tick = Instant::now();
        if self.paused_duration.is_none()
            && self.target.duration_since(self.current_tick) == Duration::ZERO
            && self.target.duration_since(self.previous_tick) != Duration::ZERO
        {
            Some(TimingEvent::TimeUp)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.target = Instant::now() + self.default_duration;
        self.paused_duration = Some(self.default_duration);
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
            if remaining_time.is_zero() {
                String::new()
            } else {
                let target = now.add(remaining_time);
                format!("Target time: {}", target.format("%H:%M:%S"))
            }
        }
    }
}
