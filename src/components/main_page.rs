use crate::App;

use super::{controls::Controls, timer::Timer, AppComponent};

pub struct MainPage;

impl AppComponent for MainPage {
    type Context = App;

    // fn add(ctx: &mut Self::Context, ui: &mut egui::Ui) {

    // }

    fn with_frame(ctx: &mut Self::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let diameter = ctx.circle.radius * 2.0;

        ui.add_space((ui.available_height() - diameter) / 2.0 - 75.0);

        Timer::add(ctx, ui);
        Controls::with_frame(ctx, ui, frame);
    }
}
