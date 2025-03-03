//! This module allows you to call another function before an existing function or to replace it with another function.
use dashmap::DashMap;
use retour::RawDetour;
use std::sync::OnceLock;

/// - Key: 'static pointer to original function
/// - value: trampoline pair fn ptr.
pub fn get_trampoline() -> &'static DashMap<usize, RawDetour> {
    static HOOKS: OnceLock<DashMap<usize, RawDetour>> = OnceLock::new();
    HOOKS.get_or_init(DashMap::new)
}

/// Sets up a trampoline to replace the original function with a new one.
///
/// # Safety
/// This is an unsafe function as it manipulates raw pointers and enables function detouring.
/// - Both the `original` and `replacement` pointers must point to valid functions with matching signatures.
/// - Undefined behavior may occur if the functions have different calling conventions or incompatible arguments.
///
/// # Errors
/// This function returns a `retour::Error` if:
/// - The detour cannot be created due to invalid function pointers.
/// - Enabling the hook fails.
///
/// # Example
/// ```
/// use retour::Error;
/// use commonlibsse_ng::skse::trampoline::add_hook;
///
/// fn add5(val: i32) -> i32 {
///     val + 5
/// }
///
/// fn add10(val: i32) -> i32 {
///     val + 10
/// }
///
/// let original = add5 as *const ();
/// let replacement = add10 as *const ();
///
/// // Verify the original behavior
/// assert_eq!(add5(5), 10);
///
/// // Replace the original function with the new one
/// unsafe { add_hook(original, replacement) }.unwrap();
/// assert_eq!(add5(5), 15);
/// ```
pub unsafe fn add_hook(original: *const (), replacement: *const ()) -> Result<(), retour::Error> {
    let detour = unsafe { RawDetour::new(original, replacement)? };
    unsafe { detour.enable() }?;
    get_trampoline().insert(original.addr(), detour);
    Ok(())
}

/// Removes a previously added hook by disabling the trampoline and restoring the original function.
///
/// # Errors
/// This function returns a `retour::Error` if:
/// - Disabling the hook fails.
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::trampoline::{add_hook, remove_hook};
/// use retour::Error;
///
/// fn add5(val: i32) -> i32 {
///     val + 5
/// }
///
/// fn add10(val: i32) -> i32 {
///     val + 10
/// }
///
/// let original = add5 as *const ();
/// let replacement = add10 as *const ();
///
/// // Verify the original behavior
/// assert_eq!(add5(5), 10);
///
/// // Replace the original function with the new one
/// unsafe { add_hook(original, replacement) }.unwrap();
/// assert_eq!(add5(5), 15); // Initially, the behavior is to add5 to add10
///
/// // Remove the hook (if added previously)
/// remove_hook(original).unwrap();
/// assert_eq!(add5(5), 10);
/// ```
pub fn remove_hook(original: *const ()) -> Result<(), retour::Error> {
    if let Some((_, detour)) = get_trampoline().remove(&original.addr()) {
        unsafe { detour.disable()? };
    }

    Ok(())
}

/// Enables a previously added hook, replacing the original function with the new one.
///
/// # Errors
/// This function returns a `retour::Error` if:
/// - Enabling the hook fails.
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::trampoline::{add_hook, remove_hook, enable_hook};
/// use retour::Error;
///
/// fn add5(val: i32) -> i32 {
///     val + 5
/// }
///
/// fn add10(val: i32) -> i32 {
///     val + 10
/// }
///
/// let original = add5 as *const ();
/// let replacement = add10 as *const ();
///
/// // Initially, the behavior is to add 5
/// assert_eq!(add5(5), 10);
///
/// // Add a hook to replace `add5` with `add10`
/// unsafe { add_hook(original, replacement) }.unwrap();
/// assert_eq!(add5(5), 15);
///
/// assert!(enable_hook(original).is_ok());
/// assert_eq!(add5(5), 15);
/// ```
pub fn enable_hook(original: *const ()) -> Result<(), retour::Error> {
    if let Some(detour) = get_trampoline().get(&original.addr()) {
        unsafe { detour.enable() }?;
    }
    Ok(())
}

/// # Description
/// Disables a previously added hook, restoring the original function's behavior.
///
/// # Errors
/// This function returns a `retour::Error` if:
/// - Disabling the hook fails.
///
/// # Example
/// ```
/// use commonlibsse_ng::skse::trampoline::{add_hook, remove_hook, disable_hook};
/// use retour::Error;
///
/// fn add5(val: i32) -> i32 {
///     val + 5
/// }
///
/// let original = add5 as *const ();
///
/// // Initially, the behavior is to add 5
/// assert_eq!(add5(5), 10);
///
/// assert!(disable_hook(original).is_ok());
/// assert_eq!(add5(5), 10);
/// ```
pub fn disable_hook(original: *const ()) -> Result<(), retour::Error> {
    if let Some(detour) = get_trampoline().get(&original.addr()) {
        unsafe { detour.disable() }?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    #[allow(clippy::fn_to_numeric_cast_any)]
    #[allow(clippy::missing_const_for_fn)]
    fn test_raw_detour_with_dashmap() -> Result<(), retour::Error> {
        #[allow(clippy::missing_const_for_fn)]
        fn add5(val: i32) -> i32 {
            val + 5
        }

        #[allow(clippy::missing_const_for_fn)]
        fn add10(val: i32) -> i32 {
            val + 10
        }

        let original = add5 as *const ();
        let replacement = add10 as *const ();

        assert_eq!(add5(5), 10);

        unsafe { add_hook(original, replacement) }?;
        assert_eq!(add5(5), 15);

        {
            let original_fn = get_trampoline().get(&original.addr()).unwrap();

            // Get `add5` fn
            let original_fn: fn(i32) -> i32 = unsafe { mem::transmute(original_fn.trampoline()) };
            assert_eq!(original_fn(5), 10);
        };

        remove_hook(original)?;
        assert_eq!(add5(5), 10);

        Ok(())
    }
}
