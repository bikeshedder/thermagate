use std::fmt;

use serde::Serialize;

use super::time::Time;

#[derive(Debug, Clone, Serialize)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}
