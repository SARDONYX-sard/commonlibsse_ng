use super::{day::GameDay, month::MonthIndex, year::YearInGame};
use chrono::{NaiveDate, NaiveDateTime};

/// NewType wrapper for `NaiveDateTime`, representing in-game date and time.
///
/// # Example
/// ```
/// use commonlibsse_ng::re::Calendar::{GameDateTime, YearInGame, MonthIndex, GameDay, Hour};
///
/// let year = YearInGame::new(2025.0);
/// let month = MonthIndex::new(2.0);   // March (0-based)
/// let day = GameDay::new(28.0);
/// let hour = Hour::new(15.5);         // 15:30
///
/// let date_time = GameDateTime::new(year, month, day, hour).unwrap();
/// assert_eq!(date_time.to_string(), "2025-03-28 15:30:00");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameDateTime(pub NaiveDateTime);

impl Default for GameDateTime {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl GameDateTime {
    /// The default in-game date: `77-01-01 00:00:00`.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::GameDateTime;
    /// assert_eq!(GameDateTime::default().to_string(), "0077-01-01 00:00:00");
    /// ```
    pub const DEFAULT: Self = Self::from_ymd(77, 1, 1);

    /// Creates a new `GameDateTime` from year, month, day, and hour components.
    ///
    /// - The month uses **0-based indexing** internally (`0 = January`, `11 = December`).
    /// - The day and hour values are clamped to their valid ranges.
    ///
    /// Returns `None` if the date or time is invalid.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::{GameDateTime, YearInGame, MonthIndex, GameDay, Hour};
    ///
    /// let year = YearInGame::new(2025.0);
    /// let month = MonthIndex::new(2.0);   // March
    /// let day = GameDay::new(15.0);
    /// let hour = Hour::new(9.75);         // 9:45
    ///
    /// let date_time = GameDateTime::new(year, month, day, hour).unwrap();
    /// assert_eq!(date_time.to_string(), "2025-03-15 09:45:00");
    ///
    /// // Invalid date returns `None`
    /// let invalid = GameDateTime::new(year, MonthIndex::new(12.0), day, hour);
    /// assert!(invalid.is_none());
    /// ```
    #[inline]
    pub fn new(year: YearInGame, month: MonthIndex, day: GameDay, hour: Hour) -> Option<Self> {
        let month = month.to_clamp_month()?; // 1-based month (1..=12)
        let day = day.to_clamp_day(month); // Clamped day
        let (hour, minute) = { (hour.to_hour(), hour.to_minutes()) };

        NaiveDate::from_ymd_opt(year.to_year(), month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, 0))
            .map(Self)
    }

    /// Creates a `NaiveDateTime` from year, month, and day components.
    ///
    /// This is a helper function to avoid using deprecated `from_ymd` directly.
    ///
    /// # Panics
    /// - Panics if the provided date or time is invalid or out of range.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::GameDateTime;
    ///
    /// let date = GameDateTime::from_ymd(2025, 3, 27);
    ///
    /// // Panics: invalid date
    /// // let invalid = from_ymd(2025, 13, 32);
    /// ```
    pub const fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => match date.and_hms_opt(0, 0, 0) {
                Some(time) => Self(time),
                None => panic!("Invalid time"),
            },
            None => panic!("invalid or out-of-range date"),
        }
    }
}

impl core::fmt::Display for GameDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// NewType wrapper for `f32` representing in-game hours(0-based).
///
/// Internally stored as a floating-point value:
/// - `0.0` → `00:00` (midnight)
/// - `15.5` → `15:30`
/// - `23.99` → nearly `23:59`
///
/// # Example
/// ```
/// use commonlibsse_ng::re::Calendar::Hour;
///
/// let hour = Hour::new(10.5);         // 10:30 AM
/// assert_eq!(hour.to_hour(), 10);
/// assert_eq!(hour.to_minutes(), 30);
///
/// let max_hour = Hour::new(23.99);    // Max valid hour
/// assert_eq!(max_hour.to_hour(), 23);
/// assert_eq!(max_hour.to_minutes(), 59);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Hour(f32);

impl Hour {
    /// Creates a new `Hour` instance.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::Hour;
    /// let hour = Hour::new(9.75);     // 9:45 AM
    /// assert_eq!(hour.to_hour(), 9);
    /// assert_eq!(hour.to_minutes(), 45);
    /// ```
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the hour component (`0..=23`).
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::Hour;
    ///
    /// let hour = Hour::new(14.25);   // 14:15
    /// assert_eq!(hour.to_hour(), 14);
    /// ```
    #[inline]
    pub const fn to_hour(self) -> u32 {
        self.0 as u32
    }

    /// Returns the minute component (`0..=59`).
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::Hour;
    ///
    /// let hour = Hour::new(12.5);    // 12:30 PM
    /// assert_eq!(hour.to_minutes(), 30);
    ///
    /// let almost_full = Hour::new(23.99);  // Nearly 23:59
    /// assert_eq!(almost_full.to_minutes(), 59);
    /// ```
    #[inline]
    pub fn to_minutes(self) -> u32 {
        (60.0 * self.0) as u32 % 60
    }
}
