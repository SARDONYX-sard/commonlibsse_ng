/// 0 based Month
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct MonthInGameRaw(f32);

impl MonthInGameRaw {
    pub const DEFAULT: Self = Self(0.0);

    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the ensured 1-based month(1..=12).
    #[inline]
    pub const fn to_valid_month(self) -> Option<u32> {
        let n = self.0 as u32;
        if n <= 11 { Some(n + 1) } else { None }
    }

    #[inline]
    pub const fn to_enum(self) -> Option<MonthInGame> {
        MonthInGame::from_u32(self.0 as u32)
    }
}

/// Represents the months of the year.
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
    /// Get the month name.
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
