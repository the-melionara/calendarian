use std::fmt::Display;

use egui::ComboBox;

pub fn enum_selection<T: Display + Copy + PartialEq>(
    ui: &mut egui::Ui,
    id_salt: &str,
    values: &[T],
    current: T
) -> T {
    let mut res = current;
    ComboBox::new(id_salt, "").selected_text(format!("{current}")).show_ui(ui, |ui| {
        for v in values {
            if ui.selectable_label(v == &current, format!("{v}")).clicked() {
                res = *v;
                ui.close_menu();
            }
        }
    });
    return res;
}
