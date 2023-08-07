use std::{
    fmt::{self, Display},
    time::{Duration, Instant},
};

pub struct Timer {
    start: Instant,
    previous_tick: Instant,
    current_tick: Instant,
    paused_duration: Option<Duration>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Instant::now(),
            previous_tick: Instant::now(),
            current_tick: Instant::now(),
            paused_duration: None,
        }
    }

    pub fn tick(&mut self) {
        self.previous_tick = self.current_tick;
        self.current_tick = Instant::now();
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.paused_duration = None;
        self.tick();
    }

    pub fn toggle_pause(&mut self) {
        self.tick();
        if self.paused_duration.is_none() {
            self.pause()
        } else {
            self.unpause()
        }
    }

    fn pause(&mut self) {
        self.paused_duration = Some(self.elapsed());
    }

    fn unpause(&mut self) {
        self.start = self.current_tick - self.paused_duration.unwrap();
        self.paused_duration = None;
    }

    pub fn elapsed(&self) -> Duration {
        self.paused_duration
            .unwrap_or(self.current_tick.duration_since(self.start))
    }

    pub fn latency(&self) -> Duration {
        self.current_tick.duration_since(self.previous_tick)
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elapsed = self.elapsed();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() - minutes * 60;
        let tenths = (elapsed.as_millis() - (elapsed.as_secs() as u128) * 1000) / 100;
        write!(f, "{:02}{:02}{:01}", minutes, seconds, tenths)
    }
}
