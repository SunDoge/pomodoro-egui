// pub mod icon;
// pub mod icon_loader;
pub mod runtime;
pub mod theme;

use crate::config::app_config::AppConfig;

use self::{runtime::Runtime, theme::Theme};
use egui::{Context, Style, Visuals};

pub struct ResourceLoader {
    theme: Theme,
    runtime: Option<Runtime>,
}

impl ResourceLoader {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            theme: Theme::new(&config.style),
            runtime: None,
        }
    }

    pub fn visuals(&self) -> &Visuals {
        self.theme.visuals()
    }

    pub fn slider(&self) -> &Style {
        self.theme.slider()
    }

    pub fn checkbox(&self) -> &Style {
        self.theme.checkbox()
    }

    // fn runtime(&self) -> &Runtime {
    //     self.runtime
    //         .as_ref()
    //         .expect("Resources runtime not allocated")
    // }

    pub fn load_runtime(&mut self, _cfg: &AppConfig, ctx: &Context) -> anyhow::Result<()> {
        let runtime = Runtime::new(ctx)?;
        self.runtime = Some(runtime);
        Ok(())
    }

    // pub fn icons(&self) -> &Icons {
    //     &self.runtime().icons
    // }
}
