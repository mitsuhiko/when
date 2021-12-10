use std::fmt;

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

/// Human readable time-of-day description.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeOfDay {
    EarlyMorning,
    Morning,
    LateMorning,
    Noon,
    Afternoon,
    EarlyEvening,
    Evening,
    LateEvening,
    Night,
}

impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimeOfDay::EarlyMorning => "early morning",
                TimeOfDay::Morning => "morning",
                TimeOfDay::LateMorning => "late morning",
                TimeOfDay::Noon => "noon",
                TimeOfDay::Afternoon => "afternoon",
                TimeOfDay::EarlyEvening => "early evening",
                TimeOfDay::Evening => "evening",
                TimeOfDay::LateEvening => "late evening",
                TimeOfDay::Night => "night",
            }
        )
    }
}

/// Given a datetime object returns a human readable time-of-day description.
pub fn get_time_of_day(dt: DateTime<Tz>) -> TimeOfDay {
    match dt.hour() {
        5 => TimeOfDay::EarlyMorning,
        6..=8 => TimeOfDay::Morning,
        9..=11 => TimeOfDay::LateMorning,
        12 => TimeOfDay::Noon,
        13..=16 => TimeOfDay::Afternoon,
        17..=18 => TimeOfDay::EarlyEvening,
        19..=20 => TimeOfDay::Evening,
        21..=22 => TimeOfDay::LateEvening,
        23 | 0..=4 => TimeOfDay::Night,
        24.. => unreachable!(),
    }
}
