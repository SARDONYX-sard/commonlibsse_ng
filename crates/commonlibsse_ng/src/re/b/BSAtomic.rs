use core::{
    ffi::c_void,
    sync::atomic::{AtomicU32, Ordering},
};

// NOTE: `BSAtomicValue<T>`(std::uint32_t) uses `AtomicU32` instead.

#[repr(C)]
pub struct BSCriticalSection {
    critical_section: windows::Win32::System::Threading::CRITICAL_SECTION,
}
const_assert_eq!(core::mem::size_of::<BSCriticalSection>(), 0x28);

#[repr(C)]
pub struct BSEventFlag {
    event: *mut c_void,
}
const_assert_eq!(core::mem::size_of::<BSEventFlag>(), 0x8);

#[repr(C)]
pub struct BSNonReentrantSpinLock {
    lock: AtomicU32,
}
const_assert_eq!(core::mem::size_of::<BSNonReentrantSpinLock>(), 0x4);

#[repr(C)]
struct BSSemaphoreBase {
    semaphore: windows::Win32::Foundation::HANDLE,
}
const_assert_eq!(core::mem::size_of::<BSSemaphoreBase>(), 0x8);

#[repr(C)]
pub struct BSSemaphore {
    _base: BSSemaphoreBase,
}
const_assert_eq!(core::mem::size_of::<BSSemaphore>(), 0x8);

impl Default for BSSemaphore {
    fn default() -> Self {
        Self::new()
    }
}

impl BSSemaphore {
    /// # Panics
    pub fn new() -> Self {
        Self {
            _base: BSSemaphoreBase {
                semaphore: unsafe {
                    windows::Win32::System::Threading::CreateSemaphoreW(
                        None,
                        0,
                        40,
                        windows::core::PCWSTR::null(),
                    )
                    .unwrap()
                },
            },
        }
    }
}

impl Drop for BSSemaphore {
    fn drop(&mut self) {
        unsafe {
            if let Err(error) = windows::Win32::Foundation::CloseHandle(self._base.semaphore) {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to close BSSemaphore's handle: {error}");
            };
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BSSpinLock {
    owning_thread: AtomicU32,
    lock_count: AtomicU32,
}
const_assert_eq!(core::mem::size_of::<BSSpinLock>(), 0x8);

impl BSSpinLock {
    pub const FAST_SPIN_THRESHOLD: usize = 10000;

    pub const fn new() -> Self {
        Self { owning_thread: AtomicU32::new(0), lock_count: AtomicU32::new(0) }
    }

    pub fn lock(&self, pause_attempts: u32) {
        let my_thread_id = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };

        // Equivalent to _mm_lfence() (memory fence), Rust has no direct equivalent.
        // This is added to ensure memory operations are completed in order before proceeding.
        std::sync::atomic::fence(Ordering::Acquire);

        if self.owning_thread.load(Ordering::SeqCst) == my_thread_id {
            self.lock_count.fetch_add(1, Ordering::SeqCst);
        } else {
            let mut attempts = 0;
            if self.lock_count.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                loop {
                    attempts += 1;
                    // Equivalent to _mm_pause() (processor-specific hint to reduce contention).
                    // Not directly available in Rust, so we simulate it by yielding the thread.
                    std::thread::yield_now();

                    if attempts >= pause_attempts {
                        let mut spin_count = 0;
                        while self
                            .lock_count
                            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed)
                            .is_ok()
                        {
                            // Simulate Sleep and Spin Threshold
                            if spin_count < 10 {
                                spin_count += 1;
                            } else {
                                std::thread::sleep(std::time::Duration::from_millis(1));
                            }
                        }
                        break;
                    }
                }
            }

            self.owning_thread.store(my_thread_id, Ordering::SeqCst);
            // Equivalent to _mm_sfence() (ensure writes are committed before proceeding).
            std::sync::atomic::fence(Ordering::Release);
        }
    }

    pub fn unlock(&self) {
        let my_thread_id = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };

        if self.owning_thread.load(Ordering::Acquire) == my_thread_id {
            if self.lock_count.load(Ordering::Acquire) == 1 {
                self.owning_thread.store(0, Ordering::Release);
                let _ = self.lock_count.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed);
            } else {
                self.lock_count.fetch_sub(1, Ordering::Release);
            }
        }
    }
}

#[repr(C)]
pub struct BSReadWriteLock {
    writer_thread: AtomicU32,
    lock: AtomicU32,
}
const_assert_eq!(core::mem::size_of::<BSReadWriteLock>(), 0x8);

impl BSReadWriteLock {
    pub const LOCK_WRITE: usize = 0x80000000;
    pub const LOCK_COUNT_MASK: usize = 0xFFFFFFF;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66976, ae_id = 68233)]
    fn lock_for_read(&self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66982, ae_id = 68239)]
    fn unlock_for_read(&self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66977, ae_id = 68234)]
    fn lock_for_write(&self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 66983, ae_id = 68240)]
    fn unlock_for_write(&self) {}
}

#[repr(C)]
pub struct BSSpinLockGuard<'a> {
    lock: &'a BSSpinLock,
}

impl<'a> BSSpinLockGuard<'a> {
    pub fn new(lock: &'a BSSpinLock) -> Self {
        lock.lock(0);
        Self { lock }
    }
}

impl Drop for BSSpinLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock();
        self.lock.lock_count.fetch_sub(1, Ordering::SeqCst);
    }
}

#[repr(C)]
pub struct BSReadLockGuard<'a> {
    lock: &'a BSReadWriteLock,
}

impl<'a> BSReadLockGuard<'a> {
    pub fn new(lock: &'a BSReadWriteLock) -> Self {
        lock.lock_for_read();
        Self { lock }
    }
}

impl Drop for BSReadLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock_for_read();
    }
}

#[repr(C)]
pub struct BSWriteLockGuard<'a> {
    lock: &'a BSReadWriteLock,
}

impl<'a> BSWriteLockGuard<'a> {
    pub fn new(lock: &'a BSReadWriteLock) -> Self {
        lock.lock_for_write();
        Self { lock }
    }
}

impl Drop for BSWriteLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock_for_write();
    }
}
