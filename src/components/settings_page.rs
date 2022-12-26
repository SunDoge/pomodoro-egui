use std::time::Duration;

use eframe::egui::{Checkbox, Color32, InnerResponse, Response, Style, Ui};

use super::AppComponent;
use crate::{
    widgets::{pomodoro_slider::PomodoroSlider, styled_slider::StyledSlider},
    App,
};

const VERTICAL_PADDING: f32 = 25.0;
const LEFT_PADDING: f32 = 25.0;

// TODO: Cache sliders in Vec
pub struct SettingsPage;

impl SettingsPage {
    fn draw_rounds_slider(ui: &mut Ui, style: &Style, color: Color32, val: &mut f64) -> Response {
        let mut response = None;
        ui.horizontal(|ui| {
            ui.add_space(25.0);
            let mut style = style.clone();
            style.visuals.widgets.inactive.bg_fill = color;
            style.visuals.widgets.active.bg_fill = color;
            style.visuals.widgets.hovered.bg_fill = color;

            let slider = StyledSlider::new("Rounds", 1.0, 16.0, val).with_style(&style);
            response = Some(ui.add(slider));
        });

        response.unwrap()
    }

    fn padding<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.horizontal(|ui| {
            ui.add_space(LEFT_PADDING);
            add(ui)
        })
    }
}

impl AppComponent for SettingsPage {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        ui.add_space(30.0);

        let slider = PomodoroSlider {
            title: "Short break".into(),
            color: ctx.config.style.circle_short_break,
            style: ctx.resources.slider(),
            duration: &mut ctx.config.pomodoro.short,
        };
        ui.add(slider);
        ui.add_space(VERTICAL_PADDING);

        let slider = PomodoroSlider {
            title: "Long break".into(),
            color: ctx.config.style.circle_long_break,
            style: ctx.resources.slider(),
            duration: &mut ctx.config.pomodoro.long,
        };
        ui.add(slider);
        ui.add_space(VERTICAL_PADDING);

        let slider = PomodoroSlider {
            title: "Focus".into(),
            color: ctx.config.style.circle_focus,
            style: ctx.resources.slider(),
            duration: &mut ctx.config.pomodoro.focus,
        };
        ui.add(slider);
        ui.add_space(VERTICAL_PADDING);

        let mut val: f64 = ctx.config.pomodoro.rounds as f64;
        let slider = Self::draw_rounds_slider(
            ui,
            ctx.resources.slider(),
            ctx.config.style.rounds,
            &mut val,
        );
        if slider.changed() {
            ctx.config.pomodoro.rounds = val as u16;
        }
        ui.add_space(VERTICAL_PADDING);

        // Self::paddig(ui, |ui| {
        //     let mut style = ctx.resources.checkbox().clone();
        //     style.spacing.icon_spacing = 0.0;
        //     ui.set_style(style);
        //     ui.add_space(-10.0);
        //     ui.monospace("Testing");
        // });
        ui.vertical_centered(|ui| {
            if ui.button("Reset Defaults").clicked() {
                ctx.config.pomodoro.focus = Duration::from_secs(60 * 25);
                ctx.config.pomodoro.short = Duration::from_secs(60 * 5);
                ctx.config.pomodoro.long = Duration::from_secs(60 * 15);
                ctx.config.pomodoro.rounds = 4;
            }
            if cfg!(debug_assertions) {
                if ui.button("Reset Debug").clicked() {
                    ctx.config.pomodoro.focus = Duration::from_secs(3);
                    ctx.config.pomodoro.short = Duration::from_secs(1);
                    ctx.config.pomodoro.long = Duration::from_secs(2);
                    ctx.config.pomodoro.rounds = 3;
                }
            }
        });

        ui.add_space(VERTICAL_PADDING);

        Self::padding(ui, |ui| {
            // ui.set_style(ctx.resources.checkbox().clone());
            let ch = Checkbox::new(&mut ctx.config.timer_notification, "Notify on timer end");
            ui.add(ch);
        });
    }
}
