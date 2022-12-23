use egui::{Color32, Stroke, TextStyle};

#[derive(Debug, Clone, Default)]
pub struct CircleConfig {
    pub radius: f32,
    pub background: Option<Stroke>,
    pub foreground: Option<Stroke>,
    pub text_main: Option<(Color32, TextStyle)>,
    pub text_additional: Option<(Color32, TextStyle)>,
}
