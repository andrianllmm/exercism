use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        if self.minutes > 60 {
            self.minutes -= 60;
            self.hours += 1;
            if self.hours > 24 {
                self.hours = 0;
            }
        }
        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
