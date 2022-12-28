use std::{path::PathBuf, time::Duration};

use crate::pomodoro::PomodoroConfig;

use serde::{Deserialize, Serialize};

use super::app_style::AppStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub pomodoro: PomodoroConfig,
    pub style: AppStyle,
    pub notification: Option<PathBuf>,
    pub muted: bool,
    pub portable: bool,
    pub always_on_top: bool,
    pub timer_notification: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let cfg = PomodoroConfig {
            focus: Duration::from_secs(10),
            long: Duration::from_secs(5),
            short: Duration::from_secs(3),
            rounds: 3,
        };

        Self {
            pomodoro: cfg,
            muted: false,
            portable: false,
            notification: Default::default(),
            style: Default::default(),
            always_on_top: false,
            timer_notification: false,
        }
    }
}
