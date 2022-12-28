use anyhow::Ok;
use egui::Context;

// use super::icon_loader::Icons;

pub struct Runtime {
    // pub icons: Icons,
}

impl Runtime {
    pub fn new(ctx: &Context) -> anyhow::Result<Self> {
        let style = ctx.style();
        // let icons = Icons::preload(ctx)?;
        ctx.set_style(style);
        Ok(Self {})
    }
}
