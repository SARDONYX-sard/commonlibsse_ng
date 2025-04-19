use crate::zeroable::Zeroable;
use core::ptr;

/// A zero-optimized option type that uses no discriminant.
///
/// `UntaggedOption<T>` represents `None` by storing a zeroed value of `T`,
/// and `Some(value)` by any non-zero `T`. This can save space, especially in FFI or low-level contexts.
///
/// # Safety
///
/// `T` **must be valid in its all-zero bit pattern**. Implementing `Zeroable` for a type that cannot
/// safely be all-zero **invokes undefined behavior**.
///
/// ## ⚠️ Undefined Behavior example:
///
/// ```rust,ignore
/// use core::num::NonZeroU32;
/// use std_fork::zeroable::Zeroable;
///
/// unsafe impl Zeroable for NonZeroU32 {} // ⚠️ Undefined Behavior!
/// ```
///
/// ## ✅ Safe example:
///
/// ```
/// use std_fork::option::UntaggedOption;
/// use std_fork::zeroable::Zeroable;
///
/// #[derive(Debug, PartialEq, Clone, Copy)]
/// #[repr(C)]
/// struct MyInt(u32);
///
/// unsafe impl Zeroable for MyInt {}
///
/// let opt = UntaggedOption::some(MyInt(123));
/// assert!(opt.is_some());
/// ```
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct UntaggedOption<T: Zeroable> {
    value: T,
}

impl<T> UntaggedOption<T>
where
    T: Zeroable,
{
    const ZEROED: T = unsafe { core::mem::zeroed() };
    /// Size of the inner type `T`.
    pub const T_SIZE: usize = core::mem::size_of::<T>();
    /// True if `T` is a zero-sized type (ZST).
    pub const IS_ZST: bool = Self::T_SIZE == 0;
    /// A constant representing `None`.
    pub const NONE: Self = Self { value: Self::ZEROED };

    /// Creates a new `UntaggedOption` containing a value.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    /// use std_fork::zeroable::Zeroable;
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq)]
    /// #[repr(transparent)]
    /// struct Wrapper(u32);
    ///
    /// unsafe impl Zeroable for Wrapper {}
    ///
    /// let opt = UntaggedOption::some(Wrapper(1));
    /// assert!(opt.is_some());
    /// ```
    #[inline]
    pub const fn some(value: T) -> Self {
        Self { value }
    }

    /// Creates a new `UntaggedOption` representing `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    ///
    /// let none = UntaggedOption::<u32>::none();
    /// assert!(none.is_none());
    /// ```
    #[inline]
    pub const fn none() -> Self {
        Self { value: Self::ZEROED }
    }

    /// Returns `true` if the option is `None`.
    ///
    /// # Note
    ///
    /// Zero-sized types (ZSTs) always return `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    ///
    /// let none = UntaggedOption::<u32>::none();
    /// assert!(none.is_none());
    ///
    /// let some = UntaggedOption::some(123u32);
    /// assert!(!some.is_none());
    /// ```
    #[inline]
    pub fn is_none(&self) -> bool {
        use core::slice::from_raw_parts;

        if Self::IS_ZST {
            return true;
        }

        unsafe {
            let value_bytes = from_raw_parts((&self.value as *const T) as *const u8, Self::T_SIZE);
            let zero_bytes = from_raw_parts((&Self::ZEROED as *const T) as *const u8, Self::T_SIZE);
            value_bytes == zero_bytes
        }
    }

    /// Returns `true` if the option contains a value.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    ///
    /// let opt = UntaggedOption::some(7i32);
    /// assert!(opt.is_some());
    ///
    /// let none = UntaggedOption::<()>::none();
    /// assert!(!none.is_some());
    /// ```
    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Takes the value out of the option, leaving a `None` in its place.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    /// use std_fork::zeroable::Zeroable;
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq)]
    /// #[repr(C)]
    /// struct MyInt(i32);
    ///
    /// unsafe impl Zeroable for MyInt {}
    ///
    /// let mut opt = UntaggedOption::some(MyInt(42));
    /// let taken = opt.take();
    ///
    /// assert_eq!(taken, Some(MyInt(42)));
    /// assert!(opt.is_none());
    /// ```
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.is_none() {
            return None;
        }

        let taken = unsafe { ptr::read(&self.value) };
        unsafe { ptr::write(&mut self.value as *mut T, Self::ZEROED) };

        Some(taken)
    }

    /// Returns a reference to the value if `Some`, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    ///
    /// let opt = UntaggedOption::some(999u32);
    /// assert_eq!(opt.as_ref(), Some(&999));
    ///
    /// let none = UntaggedOption::<u32>::none();
    /// assert_eq!(none.as_ref(), None);
    /// ```
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        if self.is_none() {
            return None;
        }

        Some(&self.value)
    }

    /// Returns a mutable reference to the value if `Some`, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    ///
    /// let mut opt = UntaggedOption::some(100i32);
    ///
    /// if let Some(val) = opt.as_mut() {
    ///     *val += 1;
    /// }
    ///
    /// assert_eq!(opt.as_ref(), Some(&101));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.is_none() {
            return None;
        }

        Some(&mut self.value)
    }
}

impl<T> From<Option<T>> for UntaggedOption<T>
where
    T: Zeroable,
{
    /// Converts a standard `Option<T>` into an `UntaggedOption<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use std_fork::option::UntaggedOption;
    ///
    /// let opt = UntaggedOption::from(Some(255u8));
    /// assert!(opt.is_some());
    ///
    /// let none = UntaggedOption::from(None::<u8>);
    /// assert!(none.is_none());
    /// ```
    #[inline]
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::some(v),
            None => Self::none(),
        }
    }
}
