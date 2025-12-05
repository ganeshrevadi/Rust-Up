#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    minutes: i32, // minutes since midnight, always 0..1439
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = hours * 60 + minutes;
        Self {
            minutes: Clock::normalize(total),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Clock::normalize(self.minutes + minutes),
        }
    }

    fn normalize(m: i32) -> i32 {
        let mins = m % 1440;          // wrap around 24 hours
        if mins < 0 { mins + 1440 }   // fix negative wrap
        else { mins }
    }
}

use std::fmt;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / 60;
        let mins = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, mins)
    }
}
