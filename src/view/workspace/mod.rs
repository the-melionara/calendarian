use std::path::PathBuf;

use calendar_view::CalendarUI;
use egui::CentralPanel;

use crate::model::project::Project;

pub mod calendar_view;

#[derive(Debug)]
pub struct WorkspaceState {
    project: Project,
    calendar: CalendarUI,
}

impl WorkspaceState {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project: Project::new(project_path),
            calendar: CalendarUI::default(),
        }
    }
    
    pub fn update(&mut self, ctx: &egui::Context, interactable: bool) {
        CentralPanel::default().show(ctx, |ui| ui.add_enabled_ui(interactable, |ui| {
            self.calendar.update(&mut self.project, ui);
        }));
    }

    pub fn project(&self) -> &Project {
        &self.project
    }
}
