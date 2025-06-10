use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Badge {
    Level100,
    Level250,
    Level500,
    MonthStreak,
    WeekStreak,
}

impl Badge {
    pub fn as_str(&self) -> &'static str {
        match self {
            Badge::Level100 => "level_100",
            Badge::Level250 => "level_250",
            Badge::Level500 => "level_500",
            Badge::MonthStreak => "month_streak",
            Badge::WeekStreak => "week_streak",
        }
    }
}
