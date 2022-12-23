#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod components;
pub mod config;
pub mod defines;
pub mod pomodoro;
pub mod serde_helpers;
pub mod widgets;

pub use app::App;
