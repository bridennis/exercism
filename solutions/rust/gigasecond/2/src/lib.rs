use std::ops::Add;

use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    DateTime::add(start, Duration::seconds(1_000_000_000))
}
