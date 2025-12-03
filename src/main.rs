#![feature(int_roundings)]

use app::Application;
use egui::ViewportBuilder;

mod app;
mod project_selection;
mod upstream;
mod modals;
mod status_bar;
mod project;
mod workspace;
mod calendar;
mod utils;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Calendarian",
        options,
        Box::new(|_| Ok(Box::<Application>::default())),
    )
}
