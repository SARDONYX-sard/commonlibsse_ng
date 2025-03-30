mod day;
mod month;
mod time;
mod year;

use core::ffi::c_char;

pub use self::day::{GameDay, Week};
pub use self::month::{MonthInGame, MonthIndex};
pub use self::time::{GameDateTime, Hour};
pub use self::year::Year;
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
    pub fn get_singleton() -> Option<&'static Self> {
        use crate::rel::{ResolvableAddress as _, id::RelocationID};

        const SE_ID: u64 = 514287;
        const AE_ID: u64 = 400447;
        let address = match RelocationID::from_se_ae_id(SE_ID, AE_ID).address() {
            Ok(address) => address,
            Err(_err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to get address(se_id={SE_ID}, ae_id={AE_ID}): {_err}");
                return None;
            }
        };

        unsafe {
            let calendar_ptr: *mut *mut Self = core::mem::transmute(address.as_ptr());

            // NOTE: Always use `read_unaligned`. If use `as_ref`, a definite crash occurs.
            let calendar_ptr = calendar_ptr.read_unaligned();
            calendar_ptr.as_ref()
        }
    }

    /// Gets the current game time.
    #[inline]
    pub fn get_current_game_time(&self) -> Option<Hour> {
        debug_assert!(crate::rex::win32::is_accessible_struct(self.game_days_passed));
        unsafe { self.game_days_passed.as_ref().map(|g| Hour::new(g.value)) }
    }

    /// Gets the current day.
    #[inline]
    pub fn get_day(&self) -> Option<GameDay> {
        debug_assert!(crate::rex::win32::is_accessible_struct(self.game_day));
        unsafe { self.game_day.as_ref().map(|g| GameDay::new(g.value)) }
    }

    /// Gets the day name.
    #[inline]
    pub fn get_day_name(&self) -> Option<&'static str> {
        self.get_day_of_week().map(|day_of_week| day_of_week.as_str())
    }

    /// Gets the day of the week.
    #[inline]
    pub fn get_day_of_week(&self) -> Option<Week> {
        self.get_days_passed().and_then(|day| day.to_week())
    }

    /// Gets the number of days passed.
    #[inline]
    pub fn get_days_passed(&self) -> Option<GameDay> {
        unsafe { self.game_days_passed.as_ref().map(|g| GameDay::new(g.value)) }
    }

    /// Gets the time string.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 35413, ae_id = 36311)]
    pub fn get_time_date_string(dest: *mut c_char, max: u32, show_year: bool) {}

    /// Gets the current hour.
    #[inline]
    pub fn get_hour(&self) -> Option<Hour> {
        unsafe { self.game_hour.as_ref().map(|g| Hour::new(g.value)) }
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
        Some(self.get_hour()?.to_minutes())
    }

    /// Gets the current month.
    #[inline]
    pub fn get_month(&self) -> Option<MonthIndex> {
        unsafe { self.game_month.as_ref().map(|g| MonthIndex::new(g.value)) }
    }

    /// Gets the month name.
    #[inline]
    pub fn get_month_name(&self) -> Option<&'static str> {
        self.get_month()?.to_enum().map(|month| month.as_str())
    }

    /// Gets the ordinal suffix for the day.
    #[inline]
    pub fn get_ordinal_suffix(&self) -> Option<&'static str> {
        Some(self.get_day()?.ordinal_suffix())
    }

    /// Gets the in-game time as a `NaiveDateTime`.
    pub fn get_time(&self) -> Option<GameDateTime> {
        let year = self.get_year()?;
        let month = self.get_month()?;
        let day = self.get_day()?;
        let hour = self.get_hour()?;
        GameDateTime::new(year, month, day, hour)
    }

    /// Gets the current time scale.
    #[inline]
    pub fn get_timescale(&self) -> f32 {
        unsafe { self.time_scale.as_ref().map_or(1.0, |g| g.value) }
    }

    /// Gets the current year.
    #[inline]
    pub fn get_year(&self) -> Option<Year> {
        unsafe { self.game_year.as_ref().map(|g| Year::new(g.value)) }
    }
}
