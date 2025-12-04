#[derive(Debug)]
pub struct Week {
    days: Vec<WeekDay>,
}

impl Week {
    pub fn new() -> Self {
        Self { days: vec![
            WeekDay::new("First"),
            WeekDay::new("Second"),
            WeekDay::new("Third"),
            WeekDay::new("Fourth"),
            WeekDay::new("Fifth"),
            WeekDay::new("Sixth"),
            WeekDay::new("Seventh"),
        ] }
    }
    
    pub fn days(&self) -> &[WeekDay] {
        &self.days
    }
}


#[derive(Debug)]
pub struct WeekDay {
    name: String,
    short: String,
}

impl WeekDay {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), short: (&name[0..name.len().min(3)]).to_uppercase() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn short(&self) -> &str {
        &self.short
    }
}
