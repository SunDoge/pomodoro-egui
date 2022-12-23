use egui::hex_color;
use egui::Color32;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AppStyle {
    pub background: Color32,
    pub foreground: Color32,
    pub circle_focus: Color32,
    pub circle_short_break: Color32,
    pub circle_long_break: Color32,
    pub rounds: Color32,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            background: hex_color!("#2f384b"),
            foreground: hex_color!("#f0ead6"),
            circle_focus: hex_color!("#f25a48"),
            circle_short_break: hex_color!("#68d6ce"),
            circle_long_break: hex_color!("#8bbd78"),
            rounds: hex_color!("#fceea7"),
        }
    }
}
