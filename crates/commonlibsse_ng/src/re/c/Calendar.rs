mod day;
mod month;
mod time;

use self::day::{Day, DayOfWeek};
use self::month::Month;
use self::time::{GameDateTime, Time};
use crate::re::TESGlobal::TESGlobal;

/// Represents the `Calendar` class from C++.
#[repr(C)]
#[derive(Debug)]
pub struct Calendar {
    pad01: u8,                        // 0x01
    pad02: u16,                       // 0x02
    pad04: u32,                       // 0x04
    game_year: *mut TESGlobal,        // 0x08
    game_month: *mut TESGlobal,       // 0x10
    game_day: *mut TESGlobal,         // 0x18
    game_hour: *mut TESGlobal,        // 0x20
    game_days_passed: *mut TESGlobal, // 0x28
    time_scale: *mut TESGlobal,       // 0x30
    midnights_passed: u32,            // 0x38
    raw_days_passed: f32,             // 0x3C
}

const _: () = {
    assert!(core::mem::size_of::<Calendar>() == 0x40);
};

impl Calendar {
    /// Returns the singleton instance of `Calendar`.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 514287, ae_id = 400447)]
    pub fn get_singleton() -> *mut Calendar {}

    /// Gets the current game time.
    #[inline]
    pub fn get_current_game_time(&self) -> Option<Time> {
        debug_assert!(crate::rex::win32::is_accessible_struct(self.game_days_passed));
        unsafe { self.game_days_passed.as_ref().map(|g| Time::new(g.value)) }
    }

    /// Gets the current day.
    #[inline]
    pub fn get_day(&self) -> Option<Day> {
        debug_assert!(crate::rex::win32::is_accessible_struct(self.game_day));
        unsafe { self.game_day.as_ref().map(|g| Day::new(g.value)) }
    }

    /// Gets the day name.
    #[inline]
    pub fn get_day_name(&self) -> Option<&'static str> {
        self.get_day_of_week().map(|day_of_week| day_of_week.as_str())
    }

    /// Gets the day of the week.
    #[inline]
    pub fn get_day_of_week(&self) -> Option<DayOfWeek> {
        self.get_days_passed().and_then(|day| day.to_day())
    }

    /// Gets the number of days passed.
    #[inline]
    pub fn get_days_passed(&self) -> Option<Day> {
        unsafe { self.game_days_passed.as_ref().map(|g| Day::new(g.value)) }
    }

    /// Gets the time in HH:MM format as a string.
    #[inline]
    pub fn get_time_date_string(&self, show_year: bool) -> Option<String> {
        let (hour, minutes) = {
            let time = self.get_hour()?;
            (time.hour(), time.minutes())
        };
        let year = if show_year { format!(" {}", self.get_year()) } else { String::new() };
        Some(format!("{hour:02}:{minutes:02}{year}"))
    }

    /// Gets the current hour.
    #[inline]
    pub fn get_hour(&self) -> Option<Time> {
        unsafe { self.game_hour.as_ref().map(|g| Time::new(g.value)) }
    }

    /// Gets the number of hours passed.
    #[inline]
    pub fn get_hours_passed(&self) -> Option<f32> {
        Some(self.get_days_passed()?.0 * 24.0)
    }

    /// Gets the number of hours per day.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 241610, ae_id = 195681)]
    pub fn get_hours_per_day() -> f32 {}

    /// Gets the current minutes.
    #[inline]
    pub fn get_minutes(&self) -> Option<u32> {
        Some(self.get_hour()?.minutes())
    }

    /// Gets the current month.
    #[inline]
    pub fn get_month(&self) -> Option<Month> {
        unsafe { self.game_month.as_ref().map(|g| Month::new(g.value)) }
    }

    /// Gets the month name.
    #[inline]
    pub fn get_month_name(&self) -> Option<&'static str> {
        self.get_month()?.to_month_of_year().map(|month| month.as_str())
    }

    /// Gets the ordinal suffix for the day.
    #[inline]
    pub fn get_ordinal_suffix(&self) -> Option<&'static str> {
        Some(self.get_day()?.ordinal_suffix())
    }

    /// Gets the in-game time as a `NaiveDateTime`.
    pub fn get_time(&self) -> Option<GameDateTime> {
        let year = self.get_year() as i32;
        let month = self.get_month()?.month()?;
        let day = self.get_day()?.clamp_day(month);
        let (hour, minute) = {
            let time = self.get_hour()?;
            (time.hour(), time.minutes())
        };
        GameDateTime::new(year, month, day, hour, minute)
    }

    /// Gets the current time scale.
    #[inline]
    pub fn get_timescale(&self) -> f32 {
        unsafe { self.time_scale.as_ref().map_or(1.0, |g| g.value) }
    }

    /// Gets the current year.
    #[inline]
    pub fn get_year(&self) -> u32 {
        unsafe { self.game_year.as_ref().map_or(77, |g| g.value as u32) }
    }
}
