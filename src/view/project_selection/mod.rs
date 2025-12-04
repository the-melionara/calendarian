use std::path::PathBuf;

use egui::CentralPanel;

use crate::upstream::{Upstream, UpstreamCmd};

#[derive(Debug, Default)]
pub struct ProjectSelectionState {
    
}

impl ProjectSelectionState {
    pub fn update(&mut self, ctx: &egui::Context, upstream: &mut Upstream, interactable: bool) {
        CentralPanel::default().show(ctx, |ui| ui.add_enabled_ui(interactable, |ui| {
            ui.heading("No project selected.");
            if ui.button("Button to start fake project!!!!!!").clicked() { // TODO: REMOVE THIS EVENTUALLY PLS
                upstream.push_cmd(UpstreamCmd::OpenProject(PathBuf::new()));
            }
        }));
    }
}
