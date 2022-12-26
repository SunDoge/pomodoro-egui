use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{defines, pomodoro::PomodoroConfig};

use serde::{Deserialize, Serialize};

use super::app_style::AppStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub pomodoro: PomodoroConfig,
    pub style: AppStyle,
    pub notification: Option<PathBuf>,
    pub muted: bool,
    pub portable: bool,
    pub always_on_top: bool,
    pub timer_notification: bool,
}

impl AppConfig {
    fn get_location(portable: bool) -> anyhow::Result<PathBuf> {
        let location = if portable {
            std::env::current_dir()?
        } else {
            match dirs::config_dir() {
                Some(v) => {
                    let path = v.join(defines::APP_NAME);
                    std::fs::create_dir_all(&path)?;
                    path
                }
                None => std::env::current_dir()?,
            }
        };

        Ok(location.join(defines::APP_CONFIG_NAME))
    }

    fn load_config(location: &Path) -> anyhow::Result<Self> {
        let config = std::fs::read_to_string(location)?;
        // println!("{}", &config);
        let cfg = serde_json::from_str(&config)?;
        Ok(cfg)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config = serde_json::to_string_pretty(&self)?;
        let location = Self::get_location(self.portable)?;
        std::fs::write(&location, &config)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let config = {
            let portable = Self::get_location(true)?;
            if let Ok(mut cfg) = Self::load_config(&portable) {
                cfg.portable = true;
                cfg
            } else {
                let system = Self::get_location(false)?;
                Self::load_config(&system)?
            }
        };

        Ok(config)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let cfg = PomodoroConfig {
            focus: Duration::from_secs(10),
            long: Duration::from_secs(5),
            short: Duration::from_secs(3),
            rounds: 3,
        };

        Self {
            pomodoro: cfg,
            muted: false,
            portable: false,
            notification: Default::default(),
            style: Default::default(),
            always_on_top: false,
            timer_notification: false,
        }
    }
}
