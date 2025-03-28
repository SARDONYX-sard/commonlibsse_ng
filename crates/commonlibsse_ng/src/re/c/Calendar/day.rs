/// Represents the days of the week.
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Week {
    #[default]
    Sundas,
    Morndas,
    Tirdas,
    Middas,
    Turdas,
    Fredas,
    Loredas,
    // Total, // unused
}

impl Week {
    /// Converts the `Week` enum into a string representation.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::Week;
    /// let day = Week::Sundas;
    /// assert_eq!(day.as_str(), "Sundas");
    /// ```
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Sundas => "Sundas",
            Self::Morndas => "Morndas",
            Self::Tirdas => "Tirdas",
            Self::Middas => "Middas",
            Self::Turdas => "Turdas",
            Self::Fredas => "Fredas",
            Self::Loredas => "Loredas",
        }
    }

    /// Converts a `u32` value into a corresponding `Week` variant.
    ///
    /// Returns `None` if the value is outside the range of valid days (0-6).
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::Week;
    /// let day = Week::from_u32(2);
    /// assert_eq!(day, Some(Week::Tirdas));
    /// ```
    ///
    #[inline]
    pub const fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            0 => Self::Sundas,
            1 => Self::Morndas,
            2 => Self::Tirdas,
            3 => Self::Middas,
            4 => Self::Turdas,
            5 => Self::Fredas,
            6 => Self::Loredas,
            _ => return None,
        })
    }
}

/// Represents a day in a 0-based game.
///
/// This usually takes the range `0.0..=30.0` range.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct GameDay(pub f32);

impl GameDay {
    /// The default `GameDay` value (0.0).
    pub const DEFAULT: Self = Self(0.0);

    /// Creates a new `GameDay` instance with the specified value.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::GameDay;
    /// let game_day = GameDay::new(5.0);
    /// assert_eq!(game_day.0, 5.0);
    /// ```
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the day of the week (0-6), 0 based value.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::GameDay;
    /// let game_day = GameDay::new(3.0);
    /// assert_eq!(game_day.day_of_week(), 3);
    /// ```
    #[inline]
    pub const fn day_of_week(&self) -> u32 {
        (self.0 as u32) % 7
    }

    /// Clamps the day value based on the month's maximum days.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::GameDay;
    /// let game_day = GameDay::new(32.0);
    /// assert_eq!(game_day.to_clamp_day(2), 28); // Sun's Dawn (28 days)
    /// ```
    #[inline]
    pub fn to_clamp_day(self, month: u32) -> u32 {
        /// Days in each month
        pub const DAYS_IN_MONTH: [u8; 12] = [
            31, // Morning Star
            28, // Sun's Dawn
            31, // First Seed
            30, // Rain's Hand
            31, // Second Seed
            30, // Midyear
            31, // Sun's Height
            31, // Last Seed
            30, // Hearthfire
            31, // Frostfall
            30, // Sun's Dusk
            31, // Evening Star
        ];
        let max_days = DAYS_IN_MONTH.get((month - 1) as usize).copied().unwrap_or(31) as u32;
        (self.0 as u32).min(max_days)
    }

    /// Returns the ordinal suffix for the day (e.g., `st`, `nd`, `rd`, `th`).
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::GameDay;
    /// let game_day = GameDay::new(21.0);
    /// assert_eq!(game_day.ordinal_suffix(), "st");
    /// ```
    #[inline]
    pub const fn ordinal_suffix(&self) -> &'static str {
        match self.0 as i32 {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        }
    }

    /// Converts `GameDay` into a `Week` enum.
    ///
    /// Returns `None` if the `GameDay` value is out of range (greater than 7.0).
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::Calendar::{GameDay, Week};
    /// let game_day = GameDay::new(1.0);
    /// assert_eq!(game_day.to_week(), Some(Week::Morndas));
    /// ```
    #[inline]
    pub fn to_week(self) -> Option<Week> {
        if self.0 >= 7.0 {
            return None;
        }

        Some(match self.day_of_week() {
            0 => Week::Sundas,
            1 => Week::Morndas,
            2 => Week::Tirdas,
            3 => Week::Middas,
            4 => Week::Turdas,
            5 => Week::Fredas,
            6 => Week::Loredas,
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_week_from_u32() {
        assert_eq!(Week::from_u32(0), Some(Week::Sundas));
        assert_eq!(Week::from_u32(1), Some(Week::Morndas));
        assert_eq!(Week::from_u32(6), Some(Week::Loredas));
        assert_eq!(Week::from_u32(7), None);
        assert_eq!(Week::from_u32(100), None);
    }

    #[test]
    fn test_game_day_to_week() {
        assert_eq!(GameDay::new(0.0).to_week(), Some(Week::Sundas));
        assert_eq!(GameDay::new(3.0).to_week(), Some(Week::Middas));
        assert_eq!(GameDay::new(6.0).to_week(), Some(Week::Loredas));
        assert_eq!(GameDay::new(7.0).to_week(), None); // Out of valid range
    }

    #[test]
    fn test_game_day_clamp() {
        assert_eq!(GameDay::new(32.0).to_clamp_day(2), 28); // February (28 days)
        assert_eq!(GameDay::new(32.0).to_clamp_day(4), 30); // April (30 days)
        assert_eq!(GameDay::new(32.0).to_clamp_day(12), 31); // December (31 days)
    }

    #[test]
    fn test_ordinal_suffix() {
        assert_eq!(GameDay::new(1.0).ordinal_suffix(), "st");
        assert_eq!(GameDay::new(2.0).ordinal_suffix(), "nd");
        assert_eq!(GameDay::new(3.0).ordinal_suffix(), "rd");
        assert_eq!(GameDay::new(4.0).ordinal_suffix(), "th");
    }
}
