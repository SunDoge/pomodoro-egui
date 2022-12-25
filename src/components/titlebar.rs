// use crate::widgets::icon_button::icon_button;
// use egui_extras::RetainedImage;

use egui::{Layout, vec2};

use crate::App;

use super::AppComponent;

pub struct Titlebar;

impl AppComponent for Titlebar {
    type Context = App;

    fn with_frame(ctx: &mut Self::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let width = ui.available_width();

        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                // let icons = ctx.resources.icons();

                // if ui.add(egui::ImageButton::new(icons.settings, vec2(64, 64))).
            });
        });
    }
}
