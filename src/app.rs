use std::collections::VecDeque;

use eframe::CreationContext;
use egui::Stroke;
use notify_rust::Notification;

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

pub enum WindowAction {
    SetMinimized(bool),
    SetFullscreen(bool),
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// add new fields, give them default values when deserializing old state
pub struct App {
    pub resources: ResourceLoader,
    pub pomodoro: Pomodoro,
    pub config: AppConfig,
    pub circle: CircleConfig,
    pub page: UiPages,
    pub fullscreen: bool,
    pub window_actions: VecDeque<WindowAction>,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {

//         }
//     }
// }

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config: AppConfig = cc
            .storage
            .map(|storage| eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default())
            .unwrap_or_default();

        Self::from_config(config, cc)
    }

    pub fn from_config(config: AppConfig, cc: &CreationContext<'_>) -> Self {
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
            window_actions: VecDeque::new(),
        }
    }

    pub fn process_timer(&mut self) {
        let status = match self.pomodoro.try_next() {
            Some(v) => v,
            None => return,
        };

        match status {
            Status::Focus => {
                self.window_actions
                    .push_back(WindowAction::SetFullscreen(false));
                Notification::new()
                    .summary("Pomodoro EGUI")
                    .body("Start FOCUS")
                    .show()
                    .expect("fail to show notification");
            }
            Status::Short | Status::Long => {
                self.window_actions
                    .push_back(WindowAction::SetMinimized(false));
                self.window_actions
                    .push_back(WindowAction::SetFullscreen(true));
                Notification::new()
                    .summary("Pomodoro EGUI")
                    .body("Start BREAK")
                    .show()
                    .expect("fail to show notification");
            }
        }

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
        self.process_timer();

        match self.window_actions.pop_front() {
            Some(window_action) => match window_action {
                WindowAction::SetFullscreen(fullscreen) => {
                    self.fullscreen = fullscreen;
                    frame.set_fullscreen(fullscreen);
                }
                WindowAction::SetMinimized(minimized) => {
                    frame.set_minimized(minimized);
                }
            },
            None => {}
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Titlebar::with_frame(self, ui, frame);

            ui.add_space(15.0);

            Topbar::add(self, ui);

            ui.add_space(15.0);

            match self.page {
                UiPages::Main => MainPage::with_frame(self, ui, frame),
                UiPages::Settings => SettingsPage::add(self, ui),
            }
        });
    }

    // fn on_close_event(&mut self) -> bool {
    //     self.config.pomodoro = *self.pomodoro.config();
    //     // dbg!(self.pomodoro.config());
    //     self.config.save().expect("Fail to save config");
    //     true
    // }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.config.pomodoro = *self.pomodoro.config();
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        self.config.style.background.into()
    }
}
