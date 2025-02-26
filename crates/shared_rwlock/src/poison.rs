// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// # Forked rust std::sync::poison(ver. 1.84.0)
// See: https://github.com/rust-lang/rust/blob/1.84.0/library/std/src/sync/poison.rs
// See Rust license detail: https://github.com/rust-lang/rust/pull/43498

use core::error::Error;
#[cfg(panic = "unwind")]
use core::sync::atomic::{AtomicBool, Ordering};
use std::fmt;
#[cfg(panic = "unwind")]
use std::thread;

pub struct Flag {
    #[cfg(panic = "unwind")]
    failed: AtomicBool,
}

// Note that the Ordering uses to access the `failed` field of `Flag` below is
// always `Relaxed`, and that's because this isn't actually protecting any data,
// it's just a flag whether we've panicked or not.
//
// The actual location that this matters is when a mutex is **locked** which is
// where we have external synchronization ensuring that we see memory
// reads/writes to this flag.
//
// As a result, if it matters, we should see the correct value for `failed` in
// all cases.

impl Flag {
    #[allow(unused)]
    #[inline]
    pub const fn new() -> Self {
        Self {
            #[cfg(panic = "unwind")]
            failed: AtomicBool::new(false),
        }
    }

    /// Checks the flag for an unguarded borrow, where we only care about existing poison.
    #[inline]
    pub fn borrow(&self) -> LockResult<()> {
        if self.get() {
            Err(PoisonError::new(()))
        } else {
            Ok(())
        }
    }

    /// Checks the flag for a guarded borrow, where we may also set poison when `done`.
    #[inline]
    pub fn guard(&self) -> LockResult<Guard> {
        let ret = Guard {
            #[cfg(panic = "unwind")]
            panicking: thread::panicking(),
        };
        if self.get() {
            Err(PoisonError::new(ret))
        } else {
            Ok(ret)
        }
    }

    #[inline]
    #[cfg(panic = "unwind")]
    pub fn done(&self, guard: &Guard) {
        if !guard.panicking && thread::panicking() {
            self.failed.store(true, Ordering::Relaxed);
        }
    }

    #[inline]
    #[cfg(not(panic = "unwind"))]
    pub fn done(&self, _guard: &Guard) {}

    #[inline]
    #[cfg(panic = "unwind")]
    pub fn get(&self) -> bool {
        self.failed.load(Ordering::Relaxed)
    }

    #[inline(always)]
    #[cfg(not(panic = "unwind"))]
    pub fn get(&self) -> bool {
        false
    }

    #[inline]
    pub fn clear(&self) {
        #[cfg(panic = "unwind")]
        self.failed.store(false, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct Guard {
    #[cfg(panic = "unwind")]
    panicking: bool,
}

/// A type of error which can be returned whenever a lock is acquired.
///
/// Both [`Mutex`]es and [`RwLock`]s are poisoned whenever a thread fails while the lock
/// is held. The precise semantics for when a lock is poisoned is documented on
/// each lock, but once a lock is poisoned then all future acquisitions will
/// return this error.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex};
/// use std::thread;
///
/// let mutex = Arc::new(Mutex::new(1));
///
/// // poison the mutex
/// let c_mutex = Arc::clone(&mutex);
/// let _ = thread::spawn(move || {
///     let mut data = c_mutex.lock().unwrap();
///     *data = 2;
///     panic!();
/// }).join();
///
/// match mutex.lock() {
///     Ok(_) => unreachable!(),
///     Err(p_err) => {
///         let data = p_err.get_ref();
///         println!("recovered: {data}");
///     }
/// };
/// ```
/// [`Mutex`]: crate::sync::Mutex
/// [`RwLock`]: crate::sync::RwLock
pub struct PoisonError<T> {
    guard: T,
    #[cfg(not(panic = "unwind"))]
    _never: !,
}

#[allow(clippy::too_long_first_doc_paragraph)]
/// An enumeration of possible errors associated with a [`TryLockResult`] which
/// can occur while trying to acquire a lock, from the [`try_lock`] method on a
/// [`Mutex`] or the [`try_read`] and [`try_write`] methods on an [`RwLock`].
///
/// [`try_lock`]: crate::sync::Mutex::try_lock
/// [`try_read`]: crate::sync::RwLock::try_read
/// [`try_write`]: crate::sync::RwLock::try_write
/// [`Mutex`]: crate::sync::Mutex
/// [`RwLock`]: crate::sync::RwLock
pub enum TryLockError<T> {
    /// The lock could not be acquired because another thread failed while holding
    /// the lock.
    Poisoned(PoisonError<T>),
    /// The lock could not be acquired at this time because the operation would
    /// otherwise block.
    WouldBlock,
}

/// A type alias for the result of a lock method which can be poisoned.
///
/// The [`Ok`] variant of this result indicates that the primitive was not
/// poisoned, and the `Guard` is contained within. The [`Err`] variant indicates
/// that the primitive was poisoned. Note that the [`Err`] variant *also* carries
/// the associated guard, and it can be acquired through the [`into_inner`]
/// method.
///
/// [`into_inner`]: PoisonError::into_inner
pub type LockResult<Guard> = Result<Guard, PoisonError<Guard>>;

/// A type alias for the result of a nonblocking locking method.
///
/// For more information, see [`LockResult`]. A `TryLockResult` doesn't
/// necessarily hold the associated guard in the [`Err`] type as the lock might not
/// have been acquired for other reasons.
pub type TryLockResult<Guard> = Result<Guard, TryLockError<Guard>>;

impl<T> fmt::Debug for PoisonError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PoisonError").finish_non_exhaustive()
    }
}

impl<T> fmt::Display for PoisonError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "poisoned lock: another task failed inside".fmt(f)
    }
}

impl<T> Error for PoisonError<T> {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "poisoned lock: another task failed inside"
    }
}

impl<T> PoisonError<T> {
    /// Creates a `PoisonError`.
    ///
    /// This is generally created by methods like [`Mutex::lock`](crate::sync::Mutex::lock)
    /// or [`RwLock::read`](crate::sync::RwLock::read).
    ///
    /// This method may panic if std was built with `panic="abort"`.
    #[cfg(panic = "unwind")]
    pub const fn new(guard: T) -> Self {
        Self { guard }
    }

    /// Creates a `PoisonError`.
    ///
    /// This is generally created by methods like [`Mutex::lock`](crate::sync::Mutex::lock)
    /// or [`RwLock::read`](crate::sync::RwLock::read).
    ///
    /// This method may panic if std was built with `panic="abort"`.
    #[cfg(not(panic = "unwind"))]
    #[stable(feature = "sync_poison", since = "1.2.0")]
    #[track_caller]
    pub fn new(_guard: T) -> PoisonError<T> {
        panic!("PoisonError created in a libstd built with panic=\"abort\"")
    }

    /// Consumes this error indicating that a lock is poisoned, returning the
    /// underlying guard to allow access regardless.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::sync::{Arc, Mutex};
    /// use std::thread;
    ///
    /// let mutex = Arc::new(Mutex::new(HashSet::new()));
    ///
    /// // poison the mutex
    /// let c_mutex = Arc::clone(&mutex);
    /// let _ = thread::spawn(move || {
    ///     let mut data = c_mutex.lock().unwrap();
    ///     data.insert(10);
    ///     panic!();
    /// }).join();
    ///
    /// let p_err = mutex.lock().unwrap_err();
    /// let data = p_err.into_inner();
    /// println!("recovered {} items", data.len());
    /// ```
    pub fn into_inner(self) -> T {
        self.guard
    }

    /// Reaches into this error indicating that a lock is poisoned, returning a
    /// reference to the underlying guard to allow access regardless.
    pub const fn get_ref(&self) -> &T {
        &self.guard
    }

    /// Reaches into this error indicating that a lock is poisoned, returning a
    /// mutable reference to the underlying guard to allow access regardless.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}

impl<T> From<PoisonError<T>> for TryLockError<T> {
    fn from(err: PoisonError<T>) -> Self {
        Self::Poisoned(err)
    }
}

impl<T> fmt::Debug for TryLockError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            #[cfg(panic = "unwind")]
            Self::Poisoned(..) => "Poisoned(..)".fmt(f),
            #[cfg(not(panic = "unwind"))]
            Self::Poisoned(ref p) => match p._never {},
            Self::WouldBlock => "WouldBlock".fmt(f),
        }
    }
}

impl<T> fmt::Display for TryLockError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            #[cfg(panic = "unwind")]
            Self::Poisoned(..) => "poisoned lock: another task failed inside",
            #[cfg(not(panic = "unwind"))]
            Self::Poisoned(ref p) => match p._never {},
            Self::WouldBlock => "try_lock failed because the operation would block",
        }
        .fmt(f)
    }
}

impl<T> Error for TryLockError<T> {
    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        match *self {
            #[cfg(panic = "unwind")]
            Self::Poisoned(ref p) => p.description(),
            #[cfg(not(panic = "unwind"))]
            Self::Poisoned(ref p) => match p._never {},
            Self::WouldBlock => "try_lock failed because the operation would block",
        }
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            #[cfg(panic = "unwind")]
            Self::Poisoned(ref p) => Some(p),
            #[cfg(not(panic = "unwind"))]
            Self::Poisoned(ref p) => match p._never {},
            _ => None,
        }
    }
}

pub fn map_result<T, U, F>(result: LockResult<T>, f: F) -> LockResult<U>
where
    F: FnOnce(T) -> U,
{
    match result {
        Ok(t) => Ok(f(t)),
        #[cfg(panic = "unwind")]
        Err(PoisonError { guard }) => Err(PoisonError::new(f(guard))),
    }
}
