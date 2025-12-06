use egui::{Align, Frame, Grid, Label, Response, RichText, ScrollArea, Sense, Stroke, UiBuilder};

use crate::{model::calendar::{day::DayVec, weeks::Week, Calendar, GlobalDayInt, GlobalWeekInt, MonthUint, YearInt}, view::{ui_tools::grid_pack, workspace::calendar_view::CalendarDisplayType}};

pub fn calendar_widget(
    ui: &mut egui::Ui,
    unit: CalendarUnit,
    padded: bool,
    style: CalendarStyle,
    calendar: &Calendar,
) {
    assert!(matches!(style, CalendarStyle::Full), "MINIMAL NOT SUPPORTED!!!");
    assert!(padded, "ONLY PADDED SUPPORTED");

    match unit {
        CalendarUnit::Day(day_vec) => todo!(),
        CalendarUnit::Week(week) => {
            let starting_day = week * calendar.week_def().days().len() as GlobalDayInt;
            let col_len = calendar.week_def().days().len() as u32;
            
            header(ui, col_len, calendar.week_def());
            body(ui, col_len, 1, |ui| {
                for i in 0..col_len {
                    atom_ui(
                        ui,
                        &format!(
                            "{}",
                            calendar.global_to_local(starting_day + i as GlobalDayInt).day + 1
                        ),
                        false
                    );
                }
            });
        },
        CalendarUnit::Month(year, month) => {
            let month_def = &calendar.months()[month as usize];

            let starting_weekday = calendar.starting_weekday_of_month(year, month);
            let col_len = calendar.week_def().days().len() as u32; 
            let row_len = (month_def.length() + starting_weekday).div_ceil(col_len);
            
            header(ui, col_len, calendar.week_def());
            body(ui, col_len, row_len, |ui| {
                let mut day = 1;
                let mut global_day = calendar.local_to_global(DayVec {
                    year: 0,
                    month,
                    day: 0
                }) - starting_weekday as GlobalDayInt;

                for _ in 0..row_len {
                    for _ in 0..col_len {
                        let dimmed = day > calendar.months()[month as usize].length() + starting_weekday
                                || day <= starting_weekday;
                        atom_ui(
                            ui,
                            &format!("{}", calendar.global_to_local(global_day).day + 1),
                            dimmed
                        );
                        day += 1;
                        global_day += 1;
                    }
                    ui.end_row();
                };
            });
        },
        CalendarUnit::Year(_) => {
            let months = &calendar.months();
            let (row_len, col_len) = grid_pack(months.len() as u32, false);
            
            body(ui, col_len, row_len, |ui| {
                for i in 0..row_len {
                    for j in 0..col_len {
                        let index = (i * col_len + j) as usize;
                        if index >= months.len() {
                            break;
                        }
                        
                        atom_ui(
                            ui,
                            months[index].name(),
                            false
                        );
                    }
                    ui.end_row();
                }
            });
        },
    };

}

#[derive(Clone, Copy, Debug)]
pub enum CalendarUnit {
    Day(DayVec),
    Week(GlobalWeekInt),
    Month(YearInt, MonthUint),
    Year(YearInt),
}

impl CalendarUnit {
    pub fn prev(&mut self, calendar: &Calendar) {
        match self {
            CalendarUnit::Day(day_vec) => todo!(),
            CalendarUnit::Week(week) => *week -= 1,
            CalendarUnit::Month(year, month) => if *month > 0 {
                *month -= 1;
            } else  {
                *month = (calendar.months().len() - 1) as u32;
                *year -= 1;
            },
            CalendarUnit::Year(year) => *year -= 1,
        }
    }

    pub fn next(&mut self, calendar: &Calendar) {
        match self {
            CalendarUnit::Day(day_vec) => todo!(),
            CalendarUnit::Week(week) => *week += 1,
            CalendarUnit::Month(year, month) => if (*month as usize) < calendar.months().len() - 1 {
                *month += 1;
            } else  {
                *month = 0;
                *year += 1;
            },
            CalendarUnit::Year(year) => *year += 1,
        }
    }

    pub fn name(&self, calendar: &Calendar) -> String {
        match self {
            CalendarUnit::Day(day_vec) => todo!(),
            CalendarUnit::Week(week) => {
                let starting_day = week * calendar.week_def().days().len() as GlobalDayInt;
                let start_date = calendar.global_to_local(starting_day);
                let end_date = calendar.global_to_local(
                    starting_day + (calendar.week_def().days().len() - 1) as GlobalDayInt
                );

                if start_date.year == end_date.year {
                    if start_date.month == end_date.month {
                        format!(
                            "{} {}",
                            calendar.months()[end_date.month as usize].name(),
                            end_date.year
                        )
                    } else {
                        format!(
                            "{} – {} {}",
                            calendar.months()[start_date.month as usize].name(),
                            calendar.months()[end_date.month as usize].name(),
                            end_date.year
                        )
                    }
                } else {
                    format!(
                        "{} {} – {} {}",
                        calendar.months()[start_date.month as usize].name(),
                        start_date.year,
                        calendar.months()[end_date.month as usize].name(),
                        end_date.year
                    )
                }
            },
            CalendarUnit::Month(year, month) => format!(
                "{} {}",
                calendar.months()[*month as usize].name(),
                year
            ),
            CalendarUnit::Year(year) => format!("{year}"),
        }
    }

    pub fn convert(&self, typ: CalendarDisplayType, calendar: &Calendar) -> Self {
        let global = match self {
            CalendarUnit::Day(day_vec) => calendar.local_to_global(*day_vec),
            CalendarUnit::Week(week) => week * calendar.week_def().days().len() as GlobalDayInt,
            CalendarUnit::Month(year, month) => calendar.local_to_global(DayVec {
                year: *year,
                month: *month,
                day: 0
            }),
            CalendarUnit::Year(year) => calendar.local_to_global(DayVec {
                year: *year,
                month: 0,
                day: 0
            }),
        };

        return match typ {
            CalendarDisplayType::Day => Self::Day(calendar.global_to_local(global)),
            CalendarDisplayType::Week => Self::Week(
                global / calendar.week_def().days().len() as GlobalDayInt
            ),
            CalendarDisplayType::Month => {
                let local = calendar.global_to_local(global);
                Self::Month(local.year, local.month)
            }
            CalendarDisplayType::Year => {
                let local = calendar.global_to_local(global);
                Self::Year(local.year)
            }
        }
    }
}

impl Default for CalendarUnit {
    fn default() -> Self {
        Self::Month(0, 0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CalendarStyle {
    Full,
    Minimal,
}

fn header(ui: &mut egui::Ui, col_len: u32, week_def: &Week) {
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

fn body(
    ui: &mut egui::Ui,
    col_len: u32,
    row_len: u32,
    atom_fn: impl Fn(&mut egui::Ui)
) {
    fn inner_body(
        ui: &mut egui::Ui,
        row_width: f32,
        row_height: f32,
        atom_fn: impl Fn(&mut egui::Ui)
    ) {
        let grid = Grid::new("calendar-body")
            .spacing((0.0, 0.0))
            .max_col_width(row_width)
            .min_row_height(row_height);
    
        grid.show(ui, atom_fn);
    }

    let row_width = ui.available_width() / col_len as f32;
    let row_height = ui.available_height() / row_len as f32;

    if row_height < row_width * 0.5 { // yes scroll
        ScrollArea::vertical().show(ui, |ui| inner_body(
            ui,
            row_width,
            row_width * 0.5,
            atom_fn,
        ));
    } else { // no scroll
        inner_body(
            ui,
            row_width,
            row_height,
            atom_fn
        );
    }
}

fn atom_ui(ui: &mut egui::Ui, label: &str, dimmed: bool) -> Response {
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
                    let mut rich_text = RichText::new(label);
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
