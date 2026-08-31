use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes) % (24 * 60);

        Self { hours: total_minutes / 60, minutes: total_minutes % 60 }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
