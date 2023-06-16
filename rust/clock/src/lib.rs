use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 1 * 60 * 24;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let into_minutes = hours * MINUTES_PER_HOUR;

        let total_minutes = into_minutes + minutes;

        let wrap_within_24_hours = total_minutes % MINUTES_PER_DAY;

        // ensures minutes be positive
        let pos_24_minutes = wrap_within_24_hours + MINUTES_PER_DAY;

        let wrap_within_24_hours = pos_24_minutes % MINUTES_PER_DAY;

        Clock(wrap_within_24_hours)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = self.0;

        write!(f, "{:02}:{:02}", minutes / 60, minutes % 60)
    }
}
