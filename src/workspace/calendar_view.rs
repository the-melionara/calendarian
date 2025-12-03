use std::fmt::Display;

use egui::{Align, Frame, Grid, Label, Layout, Response, RichText, ScrollArea, Sense, Stroke, UiBuilder};

use crate::{calendar::{months::Month, weeks::Week, Calendar, MonthUint}, project::Project, utils::ui_tools::enum_selection};

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
    month: MonthUint,
}

impl CalendarUI {
    pub fn update(&mut self, project: &mut Project, ui: &mut egui::Ui) {
        let calendar = project.calendar();
        let week_def = calendar.week_def();
        let month = &calendar.months()[self.month as usize];

        let starting_weekday = calendar.starting_weekday_of_month(0, self.month);
    
        let col_len = week_def.days().len() as u32;
        let row_len = (month.length() + starting_weekday).div_ceil(col_len);

        egui::menu::bar(ui, |ui| {
            _ = ui.button("Today");
            if ui.button("<").clicked() && self.month > 0 {
                self.month -= 1;
            }
            if ui.button(">").clicked() && (self.month as usize) < calendar.months().len() - 1 {
                self.month += 1;
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.display_type = enum_selection(
                    ui,
                    "span-selection",
                    CalendarDisplayType::VALUES,
                    self.display_type
                );
            });
        });
    
        self.calendar_header(ui, col_len, week_def);
        self.calendar_body(ui, col_len, row_len, starting_weekday, calendar);
    }

    fn calendar_body(
        &self,
        ui: &mut egui::Ui,
        col_len: u32,
        row_len: u32,
        starting_weekday: u32,
        calendar: &Calendar
    ) {
        fn inner_body(
            ui: &mut egui::Ui,
            row_width: f32,
            row_height: f32,
            row_len: u32,
            col_len: u32,
            starting_weekday: u32,
            calendar: &Calendar,
            month: usize
        ) {
            let grid = Grid::new("calendar-body")
                .spacing((0.0, 0.0))
                .max_col_width(row_width)
                .min_row_height(row_height);
        
            grid.show(ui, |ui| {
                let mut day = 1;
                for _ in 0..row_len {
                    for _ in 0..col_len {
                        day_ui(ui, day, day > calendar.months()[month].length() + starting_weekday || day <= starting_weekday);
                        day += 1;
                    }
                    ui.end_row();
                };
            });
        }

        let row_width = ui.available_width() / col_len as f32;
        let row_height = ui.available_height() / row_len as f32;

        if row_height < row_width * 0.5 { // yes scroll
            ScrollArea::vertical().show(ui, |ui| inner_body(
                ui,
                row_width,
                row_width * 0.5,
                row_len,
                col_len,
                starting_weekday,
                calendar,
                self.month as usize
            ));
        } else { // no scroll
            inner_body(
                ui,
                row_width,
                row_height,
                row_len,
                col_len,
                starting_weekday,
                calendar,
                self.month as usize
            );
        }
    }

    fn calendar_header(&self, ui: &mut egui::Ui, col_len: u32, week_def: &Week) {
        let grid = Grid::new("calendar-header")
            .spacing((0.0, 0.0))
            .max_col_width(ui.available_width() / col_len as f32)
            .min_row_height(30.0);
        
        grid.show(ui, |ui| {
            for day in week_def.days() {
                ui.vertical_centered(|ui| {
                    ui.strong(day.short());
                });
            };
            ui.end_row();
        });
    }
}

fn day_ui(ui: &mut egui::Ui, number: u32, dimmed: bool) -> Response {
    ui.scope_builder(
        UiBuilder::new()
            .id_salt("day_button")
            .sense(Sense::click()),
        |ui| {
            let response = ui.response();
            let visuals = ui.style().interact(&response);
            // let text_col = visuals.text_color();

            Frame::canvas(ui.style())
                .fill(
                    visuals.bg_fill.gamma_multiply(if dimmed { 0.1 } else { 0.3 }
                        + if response.hovered() { 0.2 } else { 0.0 })
                )
                .outer_margin(2.5)
                .corner_radius(0.0)
                .inner_margin(5.0)
                .stroke(Stroke::NONE)
                .show(ui, |ui| {
                    let mut rich_text = RichText::new(format!("{number}"));
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());
                    if dimmed {
                        rich_text = rich_text.weak();
                    }
                    ui.vertical_centered_justified(|ui|
                        ui.add(Label::new(rich_text).halign(Align::Center).selectable(false))
                    );
                })
        }
    ).response
}
