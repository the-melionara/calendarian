use crate::view::app::AppContext;

pub trait Modal {
    fn update(&mut self, ui: &mut egui::Ui, app_ctx: AppContext<'_>) -> ModalAction;
}

pub enum ModalAction {
    DoNothing,
    Close,
}
