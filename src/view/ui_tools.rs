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

pub fn grid_pack(min: u32, row_priority: bool) -> (u32, u32) {
    let mut a = 1;
    let mut b = 1;

    while a * b < min {
        if a == b {
            b += 1;
        } else {
            a = b;
        }
    }

    return if row_priority {
        (a.max(b), a.min(b))
    } else {
        (a.min(b), a.max(b))
    };
}

