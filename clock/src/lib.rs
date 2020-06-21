use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

pub trait FromClock<T> {
    fn from_clock(clock: &Clock) -> T;
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::normalize_hours(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::normalize_hours(self.hours, self.minutes + minutes)
    }

    fn normalize_hours(hours: i32, minutes: i32) -> Self {
        let overflowing_hours = if minutes < 0 && minutes % 60 != 0 {
            (minutes / 60) - 1
        } else {
            minutes / 60
        };

        Clock {
            minutes: minutes.rem_euclid(60),
            hours: (hours + overflowing_hours).rem_euclid(24),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl FromClock<String> for String {
    fn from_clock(clock: &Clock) -> String {
        format!("{:02}:{:02}", clock.hours, clock.minutes)
    }
}
