use eframe::CreationContext;

use crate::{
    components::title_bar::title_bar_ui, config::app_config::AppConfig, pomodoro::Pomodoro,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// add new fields, give them default values when deserializing old state
pub struct App {
    pomodoro: Pomodoro,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {

//         }
//     }
// }

impl App {
    /// Called once before the first frame.
    // pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

    // }

    pub fn from_config(config: AppConfig, cc: &CreationContext) -> Self {
        let pomodoro = Pomodoro::new(config.pomodoro);

        Self { pomodoro }
    }

    pub fn process_timer(&mut self) {}
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            title_bar_ui(ui, frame);
        });
    }
}