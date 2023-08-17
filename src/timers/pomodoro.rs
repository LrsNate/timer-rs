use std::ops::Add;
use std::time::{Duration, Instant};

use chrono::Local;
use phf::{phf_map, Map};

use crate::timers::timekeeper::{Timekeeper, TimingEvent};

static DEFAULT_DURATION: Map<&'static str, u64> = phf_map! {
    "WORK" => 25 * 60,
    "REST" => 5* 60
};

pub struct PomodoroTimer {
    current_phase: &'static str,
    target: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl PomodoroTimer {
    pub fn new() -> PomodoroTimer {
        PomodoroTimer {
            current_phase: "WORK",
            target: Instant::now() + Duration::from_secs(DEFAULT_DURATION["WORK"]),
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(Duration::from_secs(DEFAULT_DURATION["WORK"])),
        }
    }
}

impl Timekeeper for PomodoroTimer {
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
        self.target = Instant::now() + Duration::from_secs(DEFAULT_DURATION[self.current_phase]);
        self.paused_duration = Some(Duration::from_secs(DEFAULT_DURATION[self.current_phase]));
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
            let target = now.add(remaining_time);
            format!("Target time: {}", target.format("%H:%M:%S"))
        }
    }
}
