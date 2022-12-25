use egui::RichText;

use crate::{
    widgets::{icon_toggle::IconToggle, pomodoro_timer::PomodoroTimer},
    App,
};

use super::AppComponent;

pub struct Timer;

impl AppComponent for Timer {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        let diameter = ctx.circle.radius * 2.0;

        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                let padding = (ui.available_width() - diameter) / 2.0;

                ui.add_space(padding);

                let is_paused = !ctx.pomodoro.is_running();

                let timer = PomodoroTimer::new(is_paused, &ctx.pomodoro, &ctx.circle);
                ui.add(timer);
            });
        });

        ui.add_space(25.0);

        // ui.horizontal(|ui| {
        //     let play = &ctx.resources.icons().play;
        //     let pause = &ctx.resources.icons().pause;
        //     ui.add_space((ui.available_width() - play.width as f32) / 2.0);

        //     let btn = IconToggle::new(pause, play, ctx.pomodoro.is_running()).frame(false);
        //     if ui.add(btn).clicked() {
        //         ctx.pomodoro.toggle();
        //     };

        // });
        // ui.with_layout(egui::Layout::left_to_right(valign), add_contents)

        ui.vertical_centered(|ui| {
            let text = if ctx.pomodoro.is_running() {
                "⏸ Pause"
            } else {
                "▶ Play"
            };
            if ui.button(RichText::new(text).size(20.0)).clicked() {
                ctx.pomodoro.toggle();
            }
        });
    }
}
