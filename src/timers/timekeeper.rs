use std::time::Duration;

pub enum TimingEvent {
    TimeUp,
}

pub trait Timekeeper {
    fn tick(&mut self) -> Option<TimingEvent>;
    fn reset(&mut self);
    fn toggle_pause(&mut self);
    fn advance(&mut self) {}

    fn time(&self) -> Duration;
    fn latency(&self) -> Duration;

    fn display(&self) -> String {
        let elapsed = self.time();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() - minutes * 60;
        let tenths = (elapsed.as_millis() - (elapsed.as_secs() as u128) * 1000) / 100;
        format!("{:02}{:02}{:01}", minutes, seconds, tenths)
    }
}
