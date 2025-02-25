use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    pub const fn new(hour: u8, minute: u8) -> Option<Self> {
        if hour > 24 {
            return None;
        }
        if !matches!(minute, 0 | 15 | 30 | 45) {
            return None;
        }
        if hour == 24 && minute != 0 {
            return None;
        }
        Some(Self { hour, minute })
    }
    #[track_caller]
    pub const fn new_const(hour: u8, minute: u8) -> Self {
        match Self::new(hour, minute) {
            Some(time) => time,
            None => panic!("Invalid time"),
        }
    }
    pub fn hour(&self) -> u8 {
        self.hour
    }
    pub fn minute(&self) -> u8 {
        self.minute
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
