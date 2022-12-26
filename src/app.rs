use eframe::CreationContext;
use egui::Stroke;

use crate::{
    components::{
        main_page::MainPage, settings_page::SettingsPage, titlebar::Titlebar, topbar::Topbar,
        AppComponent, UiPages,
    },
    config::app_config::AppConfig,
    pomodoro::{Pomodoro, Status},
    resources::ResourceLoader,
    widgets::progress_circle::CircleConfig,
};

// pub type GlWindow;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// add new fields, give them default values when deserializing old state
pub struct App {
    pub resources: ResourceLoader,
    pub pomodoro: Pomodoro,
    pub config: AppConfig,
    pub circle: CircleConfig,
    pub page: UiPages,
    pub fullscreen: bool,
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
        let mut resources = ResourceLoader::new(&config);
        let style = resources.visuals().widgets.noninteractive;
        let pomodoro = Pomodoro::new(config.pomodoro);
        let circle = CircleConfig {
            background: Some((3.0, style.fg_stroke.color).into()),
            foreground: Some(Self::status_stroke(&config, Status::Focus)),
            radius: 120.0,
            ..Default::default()
        };

        resources
            .load_runtime(&config, &cc.egui_ctx)
            .expect("Failed to load Resources::Runtime");

        Self {
            resources,
            pomodoro,
            circle,
            config,
            page: UiPages::Main,
            fullscreen: false,
        }
    }

    pub fn process_timer(&mut self, frame: &mut eframe::Frame) {
        let status = match self.pomodoro.try_next() {
            Some(v) => v,
            None => return,
        };

        match status {
            Status::Focus => {
                self.fullscreen = false;
            }
            Status::Short | Status::Long => {
                self.fullscreen = true;
            }
        }
        frame.set_fullscreen(self.fullscreen);

        self.circle.foreground = Some(Self::status_stroke(&self.config, status));
    }

    pub fn status_stroke(cfg: &AppConfig, status: Status) -> Stroke {
        let color = match status {
            Status::Short => cfg.style.circle_short_break,
            Status::Long => cfg.style.circle_long_break,
            Status::Focus => cfg.style.circle_focus,
        };

        Stroke::new(10.0, color)
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.process_timer(frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Titlebar::with_frame(self, ui, frame);

            // ui.add_space(15.0);

            Topbar::add(self, ui);

            ui.add_space(15.0);

            match self.page {
                UiPages::Main => MainPage::with_frame(self, ui, frame),
                UiPages::Settings => SettingsPage::add(self, ui),
            }
        });
    }
}
