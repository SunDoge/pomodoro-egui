#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::vec2;
use pomodoro_egui::config::app_config::AppConfig;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use pomodoro_egui::defines;

    tracing_subscriber::fmt::init();

    let config = AppConfig::load().unwrap_or_default();

    let window_size = Some(vec2(360.0, 520.0));

    let native_options = eframe::NativeOptions {
        // decorated: false,
        resizable: false,
        initial_window_size: window_size,
        max_window_size: window_size,
        always_on_top: config.always_on_top,
        ..Default::default()
    };
    eframe::run_native(
        defines::APP_NAME,
        native_options,
        Box::new(|cc| Box::new(pomodoro_egui::App::from_config(config, cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(pomodoro_egui::App::new(cc))),
    )
    .expect("failed to start eframe");
}
