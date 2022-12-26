pub mod timer;

use self::timer::Timer;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Status {
    Short,
    Long,
    Focus,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Focus => "FOCUS",
            Self::Long => "LONG BREAK",
            Self::Short => "SHORT BREAK",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PomodoroConfig {
    pub short: Duration,
    pub long: Duration,
    pub focus: Duration,
    pub rounds: u16,
}

impl PomodoroConfig {
    pub fn get_duration(&self, status: &Status) -> Duration {
        match status {
            Status::Focus => self.focus,
            Status::Long => self.long,
            Status::Short => self.short,
        }
    }
}


pub struct Pomodoro {
    timer: Timer,
    config: PomodoroConfig,
    status: Status,
    round: u16,
}

impl Pomodoro {
    pub fn new(config: PomodoroConfig) -> Self {
        let timer = Timer::new_paused(config.focus);
        Self {
            timer,
            config,
            status: Status::Focus,
            round: 1,
        }
    }

    pub fn update_config(&mut self, new: PomodoroConfig) {
        if new.rounds < self.config.rounds {
            self.round = 1;
        }
        self.config = new;
        self.reset();
    }

    pub fn reset(&mut self) {
        let duration = self.config.get_duration(&self.status);
        self.timer = Timer::new_paused(duration);
    }

    pub fn pause(&mut self) {
        self.timer.pause();
    }

    pub fn resume(&mut self) {
        self.timer.resume();
    }

    pub fn toggle(&mut self) {
        if self.is_running() {
            self.pause();
        } else {
            self.resume();
        }
    }

    pub fn next(&mut self) -> Status {
        let status = match self.status {
            Status::Long => {
                self.round = 1;
                Status::Focus
            }
            Status::Short => {
                self.round += 1;
                Status::Focus
            }
            Status::Focus => {
                if self.round >= self.config.rounds {
                    Status::Long
                } else {
                    Status::Short
                }
            }
        };

        let duration = self.config.get_duration(&status);
        self.timer = Timer::new_paused(duration);
        self.status = status;

        status
    }

    pub fn try_next(&mut self) -> Option<Status> {
        if self.timer.time_left() == Duration::ZERO {
            Some(self.next())
        } else {
            None
        }
    }

    pub fn config(&self) -> &PomodoroConfig {
        &self.config
    }

    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }

    pub fn round(&self) -> u16 {
        self.round
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn time_left(&self) -> Duration {
        self.timer.time_left()
    }
}
