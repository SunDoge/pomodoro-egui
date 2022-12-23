use egui::ImageButton;
use egui_extras::RetainedImage;

// pub fn icon_button_ui(ui: &mut egui::Ui, icon: &RetainedImage) -> egui::Response {
//     let button = egui::ImageButton::new(icon.texture_id(ui.ctx()), icon.size_vec2()).frame(false);
//     let response = ui.add(button);
//     response.on_hover_cursor(egui::CursorIcon::PointingHand)
// }

// pub fn icon_button(icon: &RetainedImage) -> impl egui::Widget + '_ {
//     move |ui: &mut egui::Ui| icon_button_ui(ui, icon)
// }
