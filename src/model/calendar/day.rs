use super::{DayUint, MonthUint, YearInt};

/// MONTHS AND DAYS START AT 0
#[derive(Debug, Clone, Copy)]
pub struct DayVec {
    pub year: YearInt,
    pub month: MonthUint,
    pub day: DayUint,
}
