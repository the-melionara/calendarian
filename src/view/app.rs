use egui::ViewportCommand;

use crate::{upstream::{Upstream, UpstreamCmd}, view::{modals::Modal, project_selection::ProjectSelectionState, status_bar::StatusBar, workspace::WorkspaceState}};

#[derive(Debug)]
pub enum AppState {
    ProjectSelection(ProjectSelectionState),
    Workspace(WorkspaceState),
}

impl AppState {
    fn update(&mut self, ctx: &egui::Context, upstream: &mut Upstream, interactable: bool) {
        match self {
            AppState::ProjectSelection(x) => x.update(ctx, upstream, interactable),
            AppState::Workspace(x) => x.update(ctx, interactable),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::ProjectSelection(Default::default())
    }
}



#[derive(Default)]
pub struct Application {
    state: AppState,
    status_bar: StatusBar,
    modals: Vec<Box<dyn Modal>>,
    upstream: Upstream,
}

impl Application {
    fn make_ctx(&mut self) -> AppContext<'_> {
        AppContext { upstream: &mut self.upstream, state: &self.state }
    }

    fn handle_upstream(&mut self, ctx: &egui::Context) {
        while let Some(cmd) = self.upstream.pop_cmd() {
            match cmd {
                UpstreamCmd::TryQuit
                | UpstreamCmd::ForceQuit => ctx.send_viewport_cmd(ViewportCommand::Close),
                UpstreamCmd::OpenProject(path) => {
                    let wsp = WorkspaceState::new(path);
                    ctx.send_viewport_cmd(
                        ViewportCommand::Title(format!("{} - Calendarian", wsp.project().name()))
                    );
                    self.state = AppState::Workspace(wsp);
                },
                UpstreamCmd::TryCloseProject | UpstreamCmd::ForceCloseProject => {
                    ctx.send_viewport_cmd(ViewportCommand::Title("Calendarian".into()));
                    self.state = AppState::ProjectSelection(ProjectSelectionState::default());
                },
            }
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut interactable = true;

            // Handle modals
            let mut swap = Vec::new();
            std::mem::swap(&mut swap, &mut self.modals);
            for modal in swap.iter_mut().rev() {
                modal.update(ui, self.make_ctx());
                interactable = false;
            }
            std::mem::swap(&mut swap, &mut self.modals);

            // Handle everything else
            {
                let mut app_ctx = AppContext { upstream: &mut self.upstream, state: &self.state };
                self.status_bar.update(ctx, &mut app_ctx, interactable);
            }
            self.state.update(ctx, &mut self.upstream, interactable);
        });

        self.handle_upstream(ctx);
    }
}

pub struct AppContext<'a> {
    pub upstream: &'a mut Upstream,
    pub state: &'a AppState,
}
