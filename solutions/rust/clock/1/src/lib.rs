use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = clock_time(hours, minutes);

        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = clock_time(self.hours, self.minutes + minutes);

        Clock {
            hours: h,
            minutes: m,
        }
    }
}

fn clock_time(hours: i32, minutes: i32) -> (i32, i32) {
    let mut h = hours_to_clock_hours(hours + minutes_to_clock_hours(minutes));
    if h <= 0 {
        h += 24;
    }

    let mut m = minutes % 60;
    if m < 0 {
        m += 60;
        h -= 1;
    }

    (if h == 24 { 0 } else { h }, m)
}

fn hours_to_clock_hours(hours: i32) -> i32 {
    match hours.abs() > 24 {
        true => hours % 24,
        false => hours,
    }
}

fn minutes_to_clock_hours(minutes: i32) -> i32 {
    match minutes / 60 > 24 {
        true => minutes / 60 % 24,
        false => minutes / 60,
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
