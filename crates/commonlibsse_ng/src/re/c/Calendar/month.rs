/// 0-based Month representation.
///
/// Internally stores the month as `f32` in the range `0.0..=11.0`, corresponding to:
/// - `0.0` -> January (Morning Star)
/// - `1.0` -> February (Sun's Dawn)
/// - `11.0` -> December (Evening Star)
///
/// # Example
/// ```
/// use commonlibsse_ng::re::Calendar::MonthIndex;
///
/// let month = MonthIndex::new(0.0);
/// assert_eq!(month.to_clamp_month(), Some(1)); // 1-based month
///
/// let month = MonthIndex::new(11.0);
/// assert_eq!(month.to_clamp_month(), Some(12)); // Evening Star
///
/// let month = MonthIndex::new(12.0);
/// assert_eq!(month.to_clamp_month(), None); // Out of range
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct MonthIndex(pub f32);

impl MonthIndex {
    /// The default month value (`0.0` -> `MorningStar`).
    pub const DEFAULT: Self = Self(0.0);

    /// Creates a new `MonthIndex` instance with the specified value.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::MonthIndex;
    /// let month = MonthIndex::new(5.0);
    /// assert_eq!(month.0, 5.0);
    /// ```
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the 1-based month (1..=12) if the value is valid, otherwise `None`.
    ///
    /// - `0.0` → `1` (January)
    /// - `11.0` → `12` (December)
    ///
    /// Returns `None` if the value is out of the valid range (`0.0..=11.0`).
    ///
    /// # Example (Boundary Tests)
    /// ```
    /// # use commonlibsse_ng::re::Calendar::MonthIndex;
    /// assert_eq!(MonthIndex::new(0.0).to_clamp_month(), Some(1));    // Morning Star
    /// assert_eq!(MonthIndex::new(11.0).to_clamp_month(), Some(12));  // Evening Star
    /// assert_eq!(MonthIndex::new(12.0).to_clamp_month(), None);      // Out of range
    /// assert_eq!(MonthIndex::new(-1.0).to_clamp_month(), None);      // Out of range
    /// ```
    #[inline]
    pub const fn to_clamp_month(self) -> Option<u32> {
        let n = self.0 as u32;
        if n <= 11 { Some(n + 1) } else { None }
    }

    /// Converts `MonthIndex` into `MonthInGame` enum if the value is valid.
    ///
    /// Returns `None` if the value is out of range (`0.0..=11.0`).
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::{MonthIndex, MonthInGame};
    /// let month = MonthIndex::new(0.0);
    /// assert_eq!(month.to_enum(), Some(MonthInGame::MorningStar));
    ///
    /// let invalid_month = MonthIndex::new(12.0);
    /// assert_eq!(invalid_month.to_enum(), None);
    /// ```
    #[inline]
    pub const fn to_enum(self) -> Option<MonthInGame> {
        MonthInGame::from_u32(self.0 as u32)
    }
}

/// Represents the months of the year.
///
/// The internal values correspond to 0-based month indexing:
/// - `0` → January (`MorningStar`)
/// - `1` → February (`SunsDawn`)
/// - `11` → December (`EveningStar`)
///
/// # Example
/// ```
/// # use commonlibsse_ng::re::Calendar::MonthInGame;
/// let month = MonthInGame::FirstSeed;
/// assert_eq!(month.as_str(), "First Seed");
/// ```
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MonthInGame {
    MorningStar = 0,
    SunsDawn = 1,
    FirstSeed = 2,
    RainsHand = 3,
    SecondSeed = 4,
    Midyear = 5,
    SunsHeight = 6,
    #[default]
    LastSeed = 7,
    Hearthfire = 8,
    Frostfall = 9,
    SunsDusk = 10,
    EveningStar = 11,
    // Total, // unused
}

impl MonthInGame {
    /// Returns the string representation of the month name.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::MonthInGame;
    /// let month = MonthInGame::Midyear;
    /// assert_eq!(month.as_str(), "Midyear");
    /// ```
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::MorningStar => "Morning Star",
            Self::SunsDawn => "Sun's Dawn",
            Self::FirstSeed => "First Seed",
            Self::RainsHand => "Rain's Hand",
            Self::SecondSeed => "Second Seed",
            Self::Midyear => "Midyear",
            Self::SunsHeight => "Sun's Height",
            Self::LastSeed => "Last Seed",
            Self::Hearthfire => "Hearthfire",
            Self::Frostfall => "Frostfall",
            Self::SunsDusk => "Sun's Dusk",
            Self::EveningStar => "Evening Star",
        }
    }

    /// Converts a `u32` value into the corresponding `MonthInGame` enum.
    ///
    /// Returns `None` if the value is out of range (`0..=11`).
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::Calendar::MonthInGame;
    ///
    /// assert_eq!(MonthInGame::from_u32(0), Some(MonthInGame::MorningStar));  // January
    /// assert_eq!(MonthInGame::from_u32(11), Some(MonthInGame::EveningStar)); // December
    /// assert_eq!(MonthInGame::from_u32(12), None);                          // Out of range
    /// assert_eq!(MonthInGame::from_u32(100), None);                         // Out of range
    /// ```
    #[inline]
    pub const fn from_u32(month: u32) -> Option<Self> {
        Some(match month {
            0 => Self::MorningStar,
            1 => Self::SunsDawn,
            2 => Self::FirstSeed,
            3 => Self::RainsHand,
            4 => Self::SecondSeed,
            5 => Self::Midyear,
            6 => Self::SunsHeight,
            7 => Self::LastSeed,
            8 => Self::Hearthfire,
            9 => Self::Frostfall,
            10 => Self::SunsDusk,
            11 => Self::EveningStar,
            _ => return None,
        })
    }
}

impl From<MonthInGame> for MonthIndex {
    #[inline]
    fn from(month: MonthInGame) -> Self {
        Self(month as u32 as f32)
    }
}

impl TryFrom<MonthIndex> for MonthInGame {
    type Error = &'static str;

    #[inline]
    fn try_from(index: MonthIndex) -> Result<Self, Self::Error> {
        let u32_index = index.0 as u32;
        Self::from_u32(u32_index).ok_or("Invalid month index")
    }
}

impl core::fmt::Display for MonthInGame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
