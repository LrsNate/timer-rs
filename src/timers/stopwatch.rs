use std::time::{Duration, Instant};

use crate::timers::timekeeper::{Timekeeper, TimingEvent};

pub struct Stopwatch {
    start: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
    paused_lap_duration: Option<Duration>,
    last_lap_start: Option<Instant>,
    laps: Vec<Duration>,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start: Instant::now(),
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: Some(Duration::ZERO),
            paused_lap_duration: Some(Duration::ZERO),
            last_lap_start: None,
            laps: Vec::new(),
        }
    }
}

impl Timekeeper for Stopwatch {
    fn tick(&mut self) -> Option<TimingEvent> {
        self.previous_tick = self.current_tick;
        self.current_tick = Instant::now();
        None
    }

    fn reset(&mut self) {
        self.start = Instant::now();
        self.paused_duration = Some(Duration::ZERO);
        self.paused_lap_duration = Some(Duration::ZERO);
        self.last_lap_start = None;
        self.laps = Vec::new();
        self.tick();
    }

    fn toggle_pause(&mut self) {
        self.tick();
        if self.paused_duration.is_none() {
            self.paused_duration = Some(self.time());
            self.paused_lap_duration =
                Some(self.current_tick - self.last_lap_start.unwrap_or(self.start));
        } else {
            self.start = self.current_tick - self.paused_duration.unwrap();
            self.paused_duration = None;
            self.paused_lap_duration = None;
        }
    }

    fn lap(&mut self) {
        self.tick();
        if self.paused_duration.is_some() {
            self.laps.push(self.paused_lap_duration.unwrap());
            self.paused_lap_duration = Some(Duration::ZERO);
        } else {
            let lap_duration = self.current_tick - self.last_lap_start.unwrap_or(self.start);
            self.laps.push(lap_duration);
        }
        self.last_lap_start = Some(self.current_tick);
    }

    fn secondary_display(&self) -> String {
        let mut i = 1;
        let mut result = String::new();

        for duration in &self.laps {
            result.push_str(format!("{}. {}\n", i, format_duration(duration)).as_ref());
            i += 1;
        }

        if !result.is_empty() {
            let lap_duration = self
                .paused_lap_duration
                .unwrap_or(self.current_tick - self.last_lap_start.unwrap());
            result.push_str(format!("-> {}\n", format_duration(&lap_duration)).as_ref())
        }

        result
    }

    fn time(&self) -> Duration {
        self.paused_duration
            .unwrap_or(self.current_tick.duration_since(self.start))
    }

    fn latency(&self) -> Duration {
        self.current_tick.duration_since(self.previous_tick)
    }
}

fn format_duration(duration: &Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() - minutes * 60;
    let tenths = (duration.as_millis() - (duration.as_secs() as u128) * 1000) / 100;
    format!("{:02}:{:02}.{:01}", minutes, seconds, tenths)
}
