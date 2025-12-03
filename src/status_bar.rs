use egui::{Button, TopBottomPanel};

use crate::{app::{AppContext, AppState}, upstream::UpstreamCmd};

#[derive(Default)]
pub struct StatusBar {
    
}

impl StatusBar {
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        app_ctx: &mut AppContext<'_>,
        interactable: bool
    ) {
        TopBottomPanel::top("status_bar").show(ctx, |ui| ui.add_enabled_ui(interactable, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| self.file_dropdown(ui, app_ctx));
            });
        }));
    }

    fn file_dropdown(&mut self, ui: &mut egui::Ui, app_ctx: &mut AppContext<'_>) {
        let is_project_open = !matches!(app_ctx.state, AppState::ProjectSelection(_));
        
        ui.add(Button::new("New project"));
        ui.add(Button::new("Open project").shortcut_text("Ctrl+O"));
        ui.add(Button::new("Open recent project"));
        ui.add_enabled(is_project_open, Button::new("Save project").shortcut_text("Ctrl+S"));
        if ui.add_enabled(is_project_open, Button::new("Close project")).clicked() {
            app_ctx.upstream.push_cmd(UpstreamCmd::TryCloseProject);
            ui.close_menu()
        }
        ui.separator();
        ui.add(Button::new("Preferences"));
        ui.separator();
        if ui.add(Button::new("Exit").shortcut_text("Alt+F4")).clicked() {
            app_ctx.upstream.push_cmd(UpstreamCmd::TryQuit);
            ui.close_menu()
        }
    }
}
