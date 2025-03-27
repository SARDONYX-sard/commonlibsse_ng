/// 0 based Month
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct YearInGame(f32);

impl YearInGame {
    pub const DEFAULT: Self = Self(0.0);
    /// Year in game default value.
    pub const GAME_DEFAULT: Self = Self(77.0);

    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the ensured 1-based month(1..=12).
    #[inline]
    pub const fn to_year(self) -> i32 {
        self.0 as i32
    }
}
