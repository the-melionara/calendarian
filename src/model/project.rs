use std::path::PathBuf;

use crate::model::calendar::Calendar;

#[derive(Debug)]
pub struct Project {
    path: PathBuf,
    
    name: String,
    calendar: Calendar,
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            name: "Testing".into(),
            calendar: Calendar::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn calendar(&self) -> &Calendar {
        &self.calendar
    }
}
