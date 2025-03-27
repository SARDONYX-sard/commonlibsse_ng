use chrono::{NaiveDate, NaiveDateTime};

// Implement deprecated APIs on our own.
const fn from_ymd(year: i32, month: u32, day: u32) -> NaiveDateTime {
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => match date.and_hms_opt(0, 0, 0) {
            Some(time) => time,
            None => panic!("Invalid time"),
        },
        None => panic!("invalid or out-of-range date"),
    }
}

/// NewType wrapper for `NaiveDateTime`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameDateTime(NaiveDateTime);

impl Default for GameDateTime {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl GameDateTime {
    /// Default at compile time
    pub const DEFAULT: Self = Self(from_ymd(77, 1, 1));

    /// Creates a new `GameDateTime` from components
    #[inline]
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, 0))
            .map(Self)
    }
}

/// NewType wrapper for `f32` representing time
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Time(f32);

impl Time {
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the hour part
    #[inline]
    pub const fn hour(&self) -> u32 {
        self.0 as u32
    }

    /// Returns the minute part
    #[inline]
    pub fn minutes(&self) -> u32 {
        (60.0 * self.0) as u32 % 60
    }
}
