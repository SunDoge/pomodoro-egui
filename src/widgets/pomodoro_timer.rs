use std::{borrow::Cow, time::Duration};

use egui::Widget;

use crate::pomodoro::Pomodoro;

use super::progress_circle::{CircleConfig, ProgressCircle};

#[derive(Debug, Clone)]
pub struct PomodoroTimer<'a> {
    time_left: String,
    status: String,
    angle: f32,
    is_paused: bool,
    circle: &'a CircleConfig,
}

impl<'a> PomodoroTimer<'a> {
    pub fn new(is_paused: bool, pomodoro: &Pomodoro, circle: &'a CircleConfig) -> Self {
        let status = pomodoro.status();

        let angle = {
            let duration = pomodoro.config().get_duration(&status);
            let left = pomodoro.time_left();
            Self::map_timer(duration, left)
        };

        let time_text = Self::get_left_text(pomodoro.time_left());
        let status_text = status.to_str();

        Self {
            angle,
            circle,
            is_paused,
            time_left: time_text,
            status: status_text.to_string(),
        }
    }

    fn get_left_text(duration: Duration) -> String {
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn map_timer(duration: Duration, left: Duration) -> f32 {
        let total = duration.as_millis() as f32;
        let value = total - left.as_millis() as f32;
        let once = total / 360.0;
        value / once
    }
}

impl<'a> Widget for PomodoroTimer<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let circle = ProgressCircle {
            angle: self.angle,
            text_main: Cow::Owned(self.time_left),
            text_additional: Cow::Owned(self.status),
            config: Cow::Borrowed(self.circle),
        };

        if self.angle < 360.0 && !self.is_paused {
            ui.ctx().request_repaint();
        }

        ui.add(circle)
    }
}
