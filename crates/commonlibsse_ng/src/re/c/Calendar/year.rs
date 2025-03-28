/// Represents the 0-based year.
///
/// - Positive years represent years in the Common Era (CE).
/// - Negative years represent years Before Common Era (BCE).
///
/// # Examples
/// ```
/// use commonlibsse_ng::re::Calendar::Year;
/// let year = Year::new(77.0);
/// assert_eq!(year.to_year(), 77);  // CE
///
/// let year_bce = Year::new(-500.0);
/// assert_eq!(year_bce.to_year(), -500);  // BCE
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Year(f32);

impl Year {
    /// The default value for `Year` (`0.0`), representing the start of the Common Era (1 CE).
    pub const DEFAULT: Self = Self(0.0);

    /// The game-defined default year (`77.0`).
    pub const GAME_DEFAULT: Self = Self(77.0);

    /// Creates a new `Year` with the specified value.
    ///
    /// - Positive values indicate CE years.
    /// - Negative values indicate BCE years.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::re::Calendar::Year;
    /// let year = Year::new(105.5);
    /// assert_eq!(year.to_year(), 105);  // CE
    ///
    /// let year_bce = Year::new(-200.0);
    /// assert_eq!(year_bce.to_year(), -200);  // BCE
    /// ```
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the year as an integer ([`i32`]), truncating the decimal part.
    ///
    /// - Positive values represent CE (e.g., `2025` → `2025 CE`).
    /// - Negative values represent BCE (e.g., `-500` → `500 BCE`).
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::re::Calendar::Year;
    /// let year = Year::new(203.75);
    /// assert_eq!(year.to_year(), 203);  // CE
    ///
    /// let year_bce = Year::new(-44.9);
    /// assert_eq!(year_bce.to_year(), -44);  // BCE
    /// ```
    #[inline]
    pub const fn to_year(self) -> i32 {
        self.0 as i32
    }
}
