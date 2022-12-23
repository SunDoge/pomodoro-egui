use crate::widgets::icon_button::icon_button;
use egui_extras::RetainedImage;

pub fn title_bar_ui(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let width = ui.available_width();

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let close_icon = RetainedImage::from_svg_bytes(
                "close.svg",
                include_bytes!("../../resources/close.svg"),
            )
            .unwrap();

            if ui.add(icon_button(&close_icon)).clicked() {
                frame.close();
            }

            let minimize_icon = RetainedImage::from_svg_bytes(
                "minimize.svg",
                include_bytes!("../../resources/minimize.svg"),
            )
            .unwrap();

            if ui.add(icon_button(&minimize_icon)).clicked() {
                // frame.(false);
            }
        });
    });
}
