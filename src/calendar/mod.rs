use day::DayVec;
use months::Month;
use weeks::Week;

pub mod weeks;
pub mod months;
pub mod day;

pub type GlobalDayInt = i64;
pub type YearInt = i32;
pub type MonthUint = u32;
pub type DayUint = u32;

#[derive(Debug)]
pub struct Calendar {
    week_def: Week,
    months: Vec<Month>,

    /// Date this calendar starts in the global scope
    start_date: GlobalDayInt,
    
    /// Calendar date of start_date
    start_offset: DayUint,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            week_def: Week::new(),
            months: vec![
                Month::new("Springino", 31),
                Month::new("Summerino", 23),
                Month::new("Autumnino", 90),
                Month::new("Winterino", 15),
            ],
            start_date: 13,
            start_offset: 69,
        }
    }

    pub fn week_def(&self) -> &Week {
        &self.week_def
    }

    pub fn months(&self) -> &[Month] {
        &self.months
    }

    pub fn starting_weekday_of_month(&self, year: YearInt, month: MonthUint) -> u32 {
        let global = self.local_to_global(DayVec { year, month, day: 0 });
        return global.rem_euclid(self.week_def.days().len() as GlobalDayInt) as u32;
    }

    pub fn global_to_local(&self, global: GlobalDayInt) -> DayVec {
        let relative = global - self.start_date;
        let year_len = self.months.iter().map(|x| x.length()).sum::<u32>() as GlobalDayInt;

        let year = relative.div_floor(year_len); // TODO: TAKE INTO ACCOUNT LEAP DAYS AND NON-ZERO-YEAR CALENDARS

        // Extract month and day
        let mut day = relative - year * year_len;
        let mut month = 0;
        
        while month < self.months.len() {
            let month_len = self.length_of_month(year as YearInt, month as MonthUint);
            
            if day < month_len as GlobalDayInt {
                break;
            }
            day -= month_len as GlobalDayInt;
            month += 1;
        }

        assert!(month < self.months.len(), "HOW DID WE EVEN GET HERE");
        return DayVec { year: year as YearInt, month: month as MonthUint, day: day as DayUint };
    }

    pub fn local_to_global(&self, local: DayVec) -> GlobalDayInt {
        // Calculate day offset from start of year
        let mut day_offset = 0;
        for i in 0..local.month {
            day_offset += self.length_of_month(local.year, i) as GlobalDayInt;
        }

        let year_len = self.months.iter().map(|x| x.length()).sum::<u32>() as GlobalDayInt;

        // TODO: TAKE INTO ACCOUNT LEAP DAYS AND NON-ZERO-YEAR CALENDARS
        let days_since_ref = year_len * local.year as GlobalDayInt;
        return days_since_ref + day_offset + self.start_date;
    }

    pub fn length_of_month(&self, year: YearInt, month: MonthUint) -> u32 {
        // TODO: ACCOUNT FOR LEAP DAYS
        return self.months[month as usize].length();
    }
}
