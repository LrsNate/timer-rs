use std::collections::HashMap;
use std::ops::Add;
use std::time::{Duration, Instant};

use chrono::Local;
use serde::Deserialize;

use crate::timers::timekeeper::{Timekeeper, TimingEvent};

#[derive(Deserialize)]
pub struct PomodoroSettings {
    pub work_duration_seconds: u64,
    pub rest_duration_seconds: u64,
}

pub struct PomodoroTimer {
    default_durations: HashMap<&'static str, Duration>,
    current_phase: &'static str,
    target: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl PomodoroTimer {
    pub fn new(settings: PomodoroSettings) -> PomodoroTimer {
        let default_work_duration = Duration::from_secs(settings.work_duration_seconds);
        let default_rest_duration = Duration::from_secs(settings.rest_duration_seconds);
        let default_durations = HashMap::from([
            ("WORK", default_work_duration),
            ("REST", default_rest_duration),
        ]);
        PomodoroTimer {
            default_durations,
            current_phase: "WORK",
            target: Instant::now() + default_work_duration,
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(default_work_duration),
        }
    }
}

impl Timekeeper for PomodoroTimer {
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
        self.target = Instant::now() + self.default_durations[self.current_phase];
        self.paused_duration = Some(self.default_durations[self.current_phase]);
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

    fn advance(&mut self) {
        self.current_phase = if self.current_phase == "WORK" {
            "REST"
        } else {
            "WORK"
        };
        self.reset();
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
