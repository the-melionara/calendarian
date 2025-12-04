#[derive(Debug)]
pub struct Month {
    name: String,
    length: u32,
}

impl Month {
    pub fn new(name: &str, length: u32) -> Self {
        assert!(length > 0);
        Self { name: name.into(), length }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}
