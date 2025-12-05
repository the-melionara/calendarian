use egui::ViewportBuilder;

use crate::view::app::Application;

mod app;
mod ui_tools;
mod modals;
mod project_selection;
mod status_bar;
mod workspace;
mod widgets;

pub fn main() -> eframe::Result {
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
