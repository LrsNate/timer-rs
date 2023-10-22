use crate::timers::pomodoro::PomodoroSettings;
use crate::timers::timer::TimerSettings;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct Settings {
    pub timer: TimerSettings,
    pub pomodoro: PomodoroSettings,
}

const DEFAULT_SETTINGS: Settings = Settings {
    timer: TimerSettings {
        duration_seconds: 60,
    },
    pomodoro: PomodoroSettings {
        work_duration_seconds: 1500,
        rest_duration_seconds: 300,
    },
};

pub fn get_settings() -> Settings {
    let config_file = File::open("./timer_settings.yaml");
    if config_file.is_err() {
        return DEFAULT_SETTINGS;
    }
    let config_reader = BufReader::new(config_file.unwrap());
    let config: Result<Settings, _> = serde_yaml::from_reader(config_reader);
    config.unwrap_or(DEFAULT_SETTINGS)
}
