use egui::{vec2, RichText};

use crate::App;

use super::{AppComponent, UiPages};

pub struct Topbar;

impl AppComponent for Topbar {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add_space(15.0);

                // let settings = egui::ImageButton::new(
                //     &ctx.resources.icons().settings.texture_id(ui.ctx()),
                //     vec2(64.0, 64.0),
                // );

                // let settings = IconButton::new(&ctx.resources.icons().settings);

                // if ui.add(settings).clicked() {
                //     ctx.page = if ctx.page == UiPages::Main {
                //         UiPages::Settings
                //     } else {
                //         ctx.pomodoro.update_config(ctx.config.pomodoro);
                //         UiPages::Main
                //     }
                // }

                if ui.button(RichText::new("Settings")).clicked() {
                    ctx.page = if ctx.page == UiPages::Main {
                        UiPages::Settings
                    } else {
                        ctx.pomodoro.update_config(ctx.config.pomodoro);
                        UiPages::Main
                    }
                }
            });
        });
    }
}
