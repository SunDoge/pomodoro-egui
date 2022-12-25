use egui::Context;
use egui_extras::RetainedImage;

use crate::defines::icons::*;

use super::icon::Icon;

pub struct Icons {
    pub settings: Icon,
    pub play: Icon,
    pub pause: Icon,
}

impl Icons {
    pub fn preload(alloc: &Context) -> anyhow::Result<Self> {
        let res = Self {
            settings: Icon::from_svg(ICON_SETTINGS, [28, 28], alloc)?,
            play: Icon::from_svg(ICON_PLAY, [64, 64], alloc)?,
            pause: Icon::from_svg(ICON_PAUSE, [64, 64], alloc)?,
        };
        Ok(res)
    }
}
