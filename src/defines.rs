pub const APP_NAME: &str = "Pomodoro";
pub const APP_CONFIG_NAME: &str = "pomodoro.toml";

pub mod icons {
    pub const ICON_SETTINGS: &[u8] = include_bytes!("../resources/settings.svg");

    pub const ICON_PLAY: &[u8] = include_bytes!("../resources/play-circle.svg");
    pub const ICON_PAUSE: &[u8] = include_bytes!("../resources/pause-circle.svg");
}
