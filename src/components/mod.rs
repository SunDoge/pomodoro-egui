pub mod controls;
pub mod main_page;
pub mod settings_page;
pub mod timer;
pub mod titlebar;
pub mod topbar;

use eframe::Frame;
use egui::Ui;

#[derive(Debug, PartialEq, Eq)]
pub enum UiPages {
    Main,
    Settings,
}

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn add(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &mut Frame) {}
}
