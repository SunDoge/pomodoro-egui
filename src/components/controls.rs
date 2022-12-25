use eframe::egui::{Layout, Response, RichText, Ui};

use super::AppComponent;
// use crate::widgets::{IconButton, IconToggle};
use crate::app::App;
// use crate::resources::Icons;

pub struct Controls;

impl Controls {
    pub fn draw_rounds(ui: &mut Ui, current: u16, max: u16) -> Response {
        use eframe::egui::{CursorIcon, Label, Sense};

        let text = format!("{}/{}", current, max);
        ui.label(text);
        ui.add_space(3.0);

        let reset = RichText::new("Reset").small();
        let reset = Label::new(reset).sense(Sense::click());
        let reset = ui.add(reset);
        reset.on_hover_cursor(CursorIcon::PointingHand)
    }

    // pub fn draw_mute(ui: &mut Ui, muted: bool) -> Response {
    //     ui.add_space(15.0);

    //     // let (off, on) = (&icons.volume_off, &icons.volume_on);
    //     // let btn = IconToggle::new(off, on, muted).frame(false);

    //     // ui.add(btn)

    //     // ui.toggle_value(&mut muted, "Mute")
    // }

    // pub fn draw_skip(ui: &mut Ui) -> Response {
    //     // let btn = IconButton::new(&icons.skip);
    //     // ui.add(btn)
    //     ui.button("Skip")
    // }
}

impl AppComponent for Controls {
    type Context = App;

    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.add_space(15.0);

            ui.vertical(|ui| {
                let current = ctx.pomodoro.round();
                let max = ctx.config.pomodoro.rounds;

                if Self::draw_rounds(ui, current, max).clicked() {
                    ctx.pomodoro.reset()
                }
            });

            ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                let muted = ctx.config.muted;
                let icons = ctx.resources.icons();

                ui.add_space(15.0);

                // if Self::draw_mute(ui, icons, muted).clicked() {
                //     ctx.config.muted = !ctx.config.muted;
                // }

                // ui.add_space(5.0);

                // if Self::draw_skip(ui, icons).clicked() {
                //     let status = ctx.pomodoro.next();
                //     ctx.circle.foreground = Some(App::status_stroke(&ctx.config, status));
                // }

                if ui.toggle_value(&mut ctx.fullscreen, "Fullscreen").clicked() {
                    frame.set_fullscreen(ctx.fullscreen);
                }

                ui.add_space(5.0);

                if ui.button("Skip").clicked() {
                    let status = ctx.pomodoro.next();
                    ctx.circle.foreground = Some(App::status_stroke(&ctx.config, status));
                }
            });
        });
    }
}
