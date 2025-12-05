use std::fmt::Display;

use egui::{Align, Layout};

use crate::{model::project::Project, view::{ui_tools::enum_selection, widgets::calendar::{calendar_widget, CalendarStyle, CalendarUnit}}};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CalendarDisplayType {
    Day, Week, #[default] Month, Year,
}

impl CalendarDisplayType {
    const VALUES: &[Self] = &[Self::Day, Self::Week, Self::Month, Self::Year];
}

impl Display for CalendarDisplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalendarDisplayType::Day => write!(f, "Day"),
            CalendarDisplayType::Week => write!(f, "Week"),
            CalendarDisplayType::Month => write!(f, "Month"),
            CalendarDisplayType::Year => write!(f, "Year"),
        }
    }
}

#[derive(Debug, Default)]
pub struct CalendarUI {
    display_type: CalendarDisplayType,
    unit: CalendarUnit,
}

impl CalendarUI {
    pub fn update(&mut self, project: &mut Project, ui: &mut egui::Ui) {
        let calendar = project.calendar();

        egui::menu::bar(ui, |ui| {
            _ = ui.button("Today");
            if ui.button("<").clicked() {
                self.unit.prev(calendar);
            }
            if ui.button(">").clicked() {
                self.unit.next(calendar);
            }

            ui.label(self.unit.name(calendar));

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let new_display_type = enum_selection(
                    ui,
                    "span-selection",
                    CalendarDisplayType::VALUES,
                    self.display_type
                );

                if new_display_type != self.display_type {
                    self.unit = self.unit.convert(new_display_type);
                    self.display_type = new_display_type;
                }
            });
        });
    
        calendar_widget(
            ui,
            self.unit,
            true,
            CalendarStyle::Full,
            calendar
        );
    }
}
