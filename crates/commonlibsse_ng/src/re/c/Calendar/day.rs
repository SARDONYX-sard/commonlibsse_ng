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

/// NewType wrapper for `u32` representing days
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct GameDay(pub f32);

impl GameDay {
    pub const DEFAULT: Self = Self(0.0);

    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the day of the week (0-6)
    #[inline]
    pub const fn day_of_week(&self) -> u32 {
        (self.0 as u32) % 7
    }

    /// Clamps the day value based on the month's max days
    #[inline]
    pub fn clamp_day(&self, month: u32) -> u32 {
        /// Days in each max month
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
            31, // Evening Sta
        ];
        let max_days = DAYS_IN_MONTH.get((month - 1) as usize).copied().unwrap_or(31) as u32;
        (self.0 as u32).min(max_days)
    }

    /// Gets the ordinal suffix for the day.
    #[inline]
    pub const fn ordinal_suffix(&self) -> &'static str {
        match self.0 as i32 {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        }
    }

    /// Converts `DayValue` into `Day`
    #[inline]
    pub fn to_week(self) -> Option<Week> {
        if self.0 > 7.0 {
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
