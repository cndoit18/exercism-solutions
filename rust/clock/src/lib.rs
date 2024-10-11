use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock(i32);

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(((hours * HOUR + minutes) % DAY + DAY) % DAY)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, minutes + self.0)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.0 / HOUR, self.0 % HOUR))
    }
}
