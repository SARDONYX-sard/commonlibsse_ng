#![allow(clippy::missing_safety_doc)]
use crate::ptr::Unique;
use core::cmp::Ordering;
use core::mem::MaybeUninit;
use core::num::NonZero;
use core::ptr::NonNull;
use core::{fmt, hash, mem, ptr};

/// `*mut T` but non-zero and [covariant].
///
/// This is often the correct thing to use when building data structures using
/// raw pointers, but is ultimately more dangerous to use because of its additional
/// properties. If you're not sure if you should use `NonNull<T>`, just use `*mut T`!
///
/// Unlike `*mut T`, the pointer must always be non-null, even if the pointer
/// is never dereferenced. This is so that enums may use this forbidden value
/// as a discriminant -- `Option<NonNull<T>>` has the same size as `*mut T`.
/// However the pointer may still dangle if it isn't dereferenced.
///
/// Unlike `*mut T`, `NonNull<T>` was chosen to be covariant over `T`. This makes it
/// possible to use `NonNull<T>` when building covariant types, but introduces the
/// risk of unsoundness if used in a type that shouldn't actually be covariant.
/// (The opposite choice was made for `*mut T` even though technically the unsoundness
/// could only be caused by calling unsafe functions.)
///
/// Covariance is correct for most safe abstractions, such as `Box`, `Rc`, `Arc`, `Vec`,
/// and `LinkedList`. This is the case because they provide a public API that follows the
/// normal shared XOR mutable rules of Rust.
///
/// If your type cannot safely be covariant, you must ensure it contains some
/// additional field to provide invariance. Often this field will be a [`PhantomData`]
/// type like `PhantomData<Cell<T>>` or `PhantomData<&'a mut T>`.
///
/// Notice that `NonNull<T>` has a `From` instance for `&T`. However, this does
/// not change the fact that mutating through a (pointer derived from a) shared
/// reference is undefined behavior unless the mutation happens inside an
/// [`UnsafeCell<T>`]. The same goes for creating a mutable reference from a shared
/// reference. When using this `From` instance without an `UnsafeCell<T>`,
/// it is your responsibility to ensure that `as_mut` is never called, and `as_ptr`
/// is never used for mutation.
///
/// # Representation
///
/// Thanks to the [null pointer optimization],
/// `NonNull<T>` and `Option<NonNull<T>>`
/// are guaranteed to have the same size and alignment:
///
/// ```
/// # use std::mem::{size_of, align_of};
/// use std::ptr::NonNull;
///
/// assert_eq!(size_of::<NonNull<i16>>(), size_of::<Option<NonNull<i16>>>());
/// assert_eq!(align_of::<NonNull<i16>>(), align_of::<Option<NonNull<i16>>>());
///
/// assert_eq!(size_of::<NonNull<str>>(), size_of::<Option<NonNull<str>>>());
/// assert_eq!(align_of::<NonNull<str>>(), align_of::<Option<NonNull<str>>>());
/// ```
///
/// [covariant]: https://doc.rust-lang.org/reference/subtyping.html
/// [`PhantomData`]: core::marker::PhantomData
/// [`UnsafeCell<T>`]: core::cell::UnsafeCell
/// [null pointer optimization]: core::option#representation
#[repr(transparent)]
pub struct ConstNonNull<T: ?Sized> {
    // Remember to use `.as_ptr()` instead of `.pointer`, as field projecting to
    // this is banned by <https://github.com/rust-lang/compiler-team/issues/807>.
    pointer: NonNull<T>,
}

impl<T: Sized> ConstNonNull<T> {
    /// Creates a new `NonNull` that is dangling, but well-aligned.
    ///
    /// This is useful for initializing types which lazily allocate, like
    /// `Vec::new` does.
    ///
    /// Note that the pointer value may potentially represent a valid pointer to
    /// a `T`, which means this must not be used as a "not yet initialized"
    /// sentinel value. Types that lazily allocate must track initialization by
    /// some other means.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let ptr = NonNull::<u32>::dangling();
    /// // Important: don't try to access the value of `ptr` without
    /// // initializing it first! The pointer is not null but isn't valid either!
    /// ```
    #[must_use]
    #[inline]
    pub const fn dangling() -> Self {
        ConstNonNull { pointer: NonNull::dangling() }
    }

    /// Returns a shared references to the value. In contrast to [`as_ref`], this does not require
    /// that the value has to be initialized.
    ///
    /// For the mutable counterpart see [`as_uninit_mut`].
    ///
    /// [`as_ref`]: NonNull::as_ref
    /// [`as_uninit_mut`]: NonNull::as_uninit_mut
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that
    /// the pointer is [convertible to a reference](core::ptr#pointer-to-reference-conversion).
    /// Note that because the created reference is to `MaybeUninit<T>`, the
    /// source pointer can point to uninitialized memory.
    #[inline]
    #[must_use]
    pub const unsafe fn as_uninit_ref<'a>(self) -> &'a MaybeUninit<T> {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { &*self.cast().as_ptr() }
    }

    /// Create a new `ConstNonNull` from a `Unique`.
    pub const fn from_unique(unique: Unique<T>) -> Self {
        Self { pointer: unique.as_non_null_ptr() }
    }
}

impl<T: ?Sized> ConstNonNull<T> {
    /// Creates a new `NonNull`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = unsafe { NonNull::new_unchecked(&mut x as *mut _) };
    /// ```
    ///
    /// *Incorrect* usage of this function:
    ///
    /// ```rust,no_run
    /// use std::ptr::NonNull;
    ///
    /// // NEVER DO THAT!!! This is undefined behavior. ⚠️
    /// let ptr = unsafe { NonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
    /// ```
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *const T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        debug_assert!(!ptr.is_null(), "NonNull::new_unchecked called with null pointer");
        unsafe { ConstNonNull { pointer: NonNull::new_unchecked(ptr.cast_mut()) } }
    }

    /// Creates a new `NonNull` if `ptr` is non-null.
    ///
    /// # Panics during const evaluation
    ///
    /// This method will panic during const evaluation if the pointer cannot be
    /// determined to be null or not. See [`is_null`] for more information.
    ///
    /// [`is_null`]: ../primitive.pointer.html#method.is_null-1
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::<u32>::new(&mut x as *mut _).expect("ptr is null!");
    ///
    /// if let Some(ptr) = NonNull::<u32>::new(std::ptr::null_mut()) {
    ///     unreachable!();
    /// }
    /// ```
    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub const fn new(ptr: *mut T) -> Option<Self> {
        if !ptr.is_null() {
            // SAFETY: The pointer is already checked and is not null
            Some(unsafe { Self::new_unchecked(ptr) })
        } else {
            None
        }
    }

    /// Converts a reference to a `NonNull` pointer.
    #[inline]
    pub const fn from_ref(r: &T) -> Self {
        // SAFETY: A reference cannot be null.
        Self { pointer: unsafe { NonNull::new_unchecked((r as *const T).cast_mut()) } }
    }

    /// Gets the "address" portion of the pointer.
    ///
    /// For more details, see the equivalent method on a raw pointer, `pointer::addr`.
    ///
    /// This is a [Strict Provenance][core::ptr#strict-provenance] API.
    #[must_use]
    #[inline]
    pub fn addr(self) -> NonZero<usize> {
        // SAFETY: The pointer is guaranteed by the type to be non-null,
        // meaning that the address will be non-zero.
        unsafe { NonZero::new_unchecked(self.as_ptr().addr()) }
    }

    /// Creates a new pointer with the given address and the [provenance][core::ptr#provenance] of
    /// `self`.
    ///
    /// For more details, see the equivalent method on a raw pointer, `pointer::with_addr`.
    ///
    /// This is a [Strict Provenance][core::ptr#strict-provenance] API.
    #[must_use]
    #[inline]
    pub fn with_addr(self, addr: NonZero<usize>) -> Self {
        // SAFETY: The result of `ptr::from::with_addr` is non-null because `addr` is guaranteed to be non-zero.
        unsafe { ConstNonNull::new_unchecked(self.as_ptr().with_addr(addr.get()) as *mut _) }
    }

    /// Creates a new pointer by mapping `self`'s address to a new one, preserving the
    /// [provenance][core::ptr#provenance] of `self`.
    ///
    /// For more details, see the equivalent method on a raw pointer, `pointer::map_addr`.
    ///
    /// This is a [Strict Provenance][core::ptr#strict-provenance] API.
    #[must_use]
    #[inline]
    pub fn map_addr(self, f: impl FnOnce(NonZero<usize>) -> NonZero<usize>) -> Self {
        self.with_addr(f(self.addr()))
    }

    /// Acquires the underlying `*mut` pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::new(&mut x).expect("ptr is null!");
    ///
    /// let x_value = unsafe { *ptr.as_ptr() };
    /// assert_eq!(x_value, 0);
    ///
    /// unsafe { *ptr.as_ptr() += 2; }
    /// let x_value = unsafe { *ptr.as_ptr() };
    /// assert_eq!(x_value, 2);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_ptr(self) -> *const T {
        // This is a transmute for the same reasons as `NonZero::get`.

        // SAFETY: `NonNull` is `transparent` over a `*const T`, and `*const T`
        // and `*mut T` have the same layout, so transitively we can transmute
        // our `NonNull` to a `*mut T` directly.
        unsafe { mem::transmute::<Self, *const T>(self) }
    }

    /// Returns a shared reference to the value. If the value may be uninitialized, [`as_uninit_ref`]
    /// must be used instead.
    ///
    /// For the mutable counterpart see [`as_mut`].
    ///
    /// [`as_uninit_ref`]: NonNull::as_uninit_ref
    /// [`as_mut`]: NonNull::as_mut
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that
    /// the pointer is [convertible to a reference](core::ptr#pointer-to-reference-conversion).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::new(&mut x as *mut _).expect("ptr is null!");
    ///
    /// let ref_x = unsafe { ptr.as_ref() };
    /// println!("{ref_x}");
    /// ```
    ///
    /// [the module documentation]: core::ptr#safety
    #[must_use]
    #[inline(always)]
    pub const unsafe fn as_ref<'a>(&self) -> &'a T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        // `cast_const` avoids a mutable raw pointer deref.
        unsafe { self.pointer.as_ref() }
    }

    /// Casts to a pointer of another type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::new(&mut x as *mut _).expect("null pointer");
    ///
    /// let casted_ptr = ptr.cast::<i8>();
    /// let raw_ptr: *mut i8 = casted_ptr.as_ptr();
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn cast<U>(self) -> ConstNonNull<U> {
        // SAFETY: `self` is a `NonNull` pointer which is necessarily non-null
        ConstNonNull { pointer: self.pointer.cast() }
    }

    /// Adds an offset to a pointer.
    ///
    /// `count` is in units of T; e.g., a `count` of 3 represents a pointer
    /// offset of `3 * size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined Behavior:
    ///
    /// * The computed offset, `count * size_of::<T>()` bytes, must not overflow `isize`.
    ///
    /// * If the computed offset is non-zero, then `self` must be derived from a pointer to some
    ///   [allocated object], and the entire memory range between `self` and the result must be in
    ///   bounds of that allocated object. In particular, this range must not "wrap around" the edge
    ///   of the address space.
    ///
    /// Allocated objects can never be larger than `isize::MAX` bytes, so if the computed offset
    /// stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement.
    /// This implies, for instance, that `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always
    /// safe.
    ///
    /// [allocated object]: core::ptr#allocated-object
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut s = [1, 2, 3];
    /// let ptr: NonNull<u32> = NonNull::new(s.as_mut_ptr()).unwrap();
    ///
    /// unsafe {
    ///     println!("{}", ptr.offset(1).read());
    ///     println!("{}", ptr.offset(2).read());
    /// }
    /// ```
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    #[must_use = "returns a new pointer rather than modifying its argument"]
    pub const unsafe fn offset(self, count: isize) -> Self
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset`.
        // Additionally safety contract of `offset` guarantees that the resulting pointer is
        // pointing to an allocation, there can't be an allocation at null, thus it's safe to
        // construct `NonNull`.
        unsafe { ConstNonNull { pointer: self.pointer.offset(count) } }
    }

    /// Adds an offset to a pointer (convenience for `.offset(count as isize)`).
    ///
    /// `count` is in units of T; e.g., a `count` of 3 represents a pointer
    /// offset of `3 * size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined Behavior:
    ///
    /// * The computed offset, `count * size_of::<T>()` bytes, must not overflow `isize`.
    ///
    /// * If the computed offset is non-zero, then `self` must be derived from a pointer to some
    ///   [allocated object], and the entire memory range between `self` and the result must be in
    ///   bounds of that allocated object. In particular, this range must not "wrap around" the edge
    ///   of the address space.
    ///
    /// Allocated objects can never be larger than `isize::MAX` bytes, so if the computed offset
    /// stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement.
    /// This implies, for instance, that `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always
    /// safe.
    ///
    /// [allocated object]: core::ptr#allocated-object
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let s: &str = "123";
    /// let ptr: NonNull<u8> = NonNull::new(s.as_ptr().cast_mut()).unwrap();
    ///
    /// unsafe {
    ///     println!("{}", ptr.add(1).read() as char);
    ///     println!("{}", ptr.add(2).read() as char);
    /// }
    /// ```
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    #[must_use = "returns a new pointer rather than modifying its argument"]
    pub const unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset`.
        // Additionally safety contract of `offset` guarantees that the resulting pointer is
        // pointing to an allocation, there can't be an allocation at null, thus it's safe to
        // construct `NonNull`.
        unsafe { ConstNonNull { pointer: self.pointer.add(count) } }
    }

    /// Calculates the offset from a pointer in bytes (convenience for `.byte_offset(count as isize)`).
    ///
    /// `count` is in units of bytes.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [`add`][NonNull::add] on it. See that method for documentation
    /// and safety requirements.
    ///
    /// For non-`Sized` pointees this operation changes only the data pointer,
    /// leaving the metadata untouched.
    ///
    /// # Safety
    /// valid pointer
    #[must_use]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_add(self, count: usize) -> Self {
        // SAFETY: the caller must uphold the safety contract for `add` and `byte_add` has the same
        // safety contract.
        // Additionally safety contract of `add` guarantees that the resulting pointer is pointing
        // to an allocation, there can't be an allocation at null, thus it's safe to construct
        // `NonNull`.
        unsafe { ConstNonNull { pointer: self.pointer.byte_add(count) } }
    }

    /// Subtracts an offset from a pointer (convenience for
    /// `.offset((count as isize).wrapping_neg())`).
    ///
    /// `count` is in units of T; e.g., a `count` of 3 represents a pointer
    /// offset of `3 * size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined Behavior:
    ///
    /// * The computed offset, `count * size_of::<T>()` bytes, must not overflow `isize`.
    ///
    /// * If the computed offset is non-zero, then `self` must be derived from a pointer to some
    ///   [allocated object], and the entire memory range between `self` and the result must be in
    ///   bounds of that allocated object. In particular, this range must not "wrap around" the edge
    ///   of the address space.
    ///
    /// Allocated objects can never be larger than `isize::MAX` bytes, so if the computed offset
    /// stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement.
    /// This implies, for instance, that `vec.as_ptr().add(vec.len())` (for `vec: Vec<T>`) is always
    /// safe.
    ///
    /// [allocated object]: core::ptr#allocated-object
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let s: &str = "123";
    ///
    /// unsafe {
    ///     let end: NonNull<u8> = NonNull::new(s.as_ptr().cast_mut()).unwrap().add(3);
    ///     println!("{}", end.sub(1).read() as char);
    ///     println!("{}", end.sub(2).read() as char);
    /// }
    /// ```
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    #[must_use = "returns a new pointer rather than modifying its argument"]
    pub const unsafe fn sub(self, count: usize) -> Self
    where
        T: Sized,
    {
        unsafe { ConstNonNull { pointer: self.pointer.sub(count) } }
    }

    /// Calculates the offset from a pointer in bytes (convenience for
    /// `.byte_offset((count as isize).wrapping_neg())`).
    ///
    /// `count` is in units of bytes.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [`sub`][NonNull::sub] on it. See that method for documentation
    /// and safety requirements.
    ///
    /// For non-`Sized` pointees this operation changes only the data pointer,
    /// leaving the metadata untouched.
    ///
    /// # Safety
    /// valid pointer
    #[must_use]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_sub(self, count: usize) -> Self {
        // SAFETY: the caller must uphold the safety contract for `sub` and `byte_sub` has the same
        // safety contract.
        // Additionally safety contract of `sub` guarantees that the resulting pointer is pointing
        // to an allocation, there can't be an allocation at null, thus it's safe to construct
        // `NonNull`.
        unsafe { ConstNonNull { pointer: self.pointer.byte_sub(count) } }
    }

    /// Calculates the distance between two pointers within the same allocation. The returned value is in
    /// units of T: the distance in bytes divided by `mem::size_of::<T>()`.
    ///
    /// This is equivalent to `(self as isize - origin as isize) / (mem::size_of::<T>() as isize)`,
    /// except that it has a lot more opportunities for UB, in exchange for the compiler
    /// better understanding what you are doing.
    ///
    /// The primary motivation of this method is for computing the `len` of an array/slice
    /// of `T` that you are currently representing as a "start" and "end" pointer
    /// (and "end" is "one past the end" of the array).
    /// In that case, `end.offset_from(start)` gets you the length of the array.
    ///
    /// All of the following safety requirements are trivially satisfied for this usecase.
    ///
    /// [`offset`]: #method.offset
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined Behavior:
    ///
    /// * `self` and `origin` must either
    ///
    ///   * point to the same address, or
    ///   * both be *derived from* a pointer to the same [allocated object], and the memory range between
    ///     the two pointers must be in bounds of that object. (See below for an example.)
    ///
    /// * The distance between the pointers, in bytes, must be an exact multiple
    ///   of the size of `T`.
    ///
    /// As a consequence, the absolute distance between the pointers, in bytes, computed on
    /// mathematical integers (without "wrapping around"), cannot overflow an `isize`. This is
    /// implied by the in-bounds requirement, and the fact that no allocated object can be larger
    /// than `isize::MAX` bytes.
    ///
    /// The requirement for pointers to be derived from the same allocated object is primarily
    /// needed for `const`-compatibility: the distance between pointers into *different* allocated
    /// objects is not known at compile-time. However, the requirement also exists at
    /// runtime and may be exploited by optimizations. If you wish to compute the difference between
    /// pointers that are not guaranteed to be from the same allocation, use `(self as isize -
    /// origin as isize) / mem::size_of::<T>()`.
    // FIXME: recommend `addr()` instead of `as usize` once that is stable.
    ///
    /// [`add`]: #method.add
    /// [allocated object]: core::ptr#allocated-object
    ///
    /// # Panics
    ///
    /// This function panics if `T` is a Zero-Sized Type ("ZST").
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let a = [0; 5];
    /// let ptr1: NonNull<u32> = NonNull::from(&a[1]);
    /// let ptr2: NonNull<u32> = NonNull::from(&a[3]);
    /// unsafe {
    ///     assert_eq!(ptr2.offset_from(ptr1), 2);
    ///     assert_eq!(ptr1.offset_from(ptr2), -2);
    ///     assert_eq!(ptr1.offset(2), ptr2);
    ///     assert_eq!(ptr2.offset(-2), ptr1);
    /// }
    /// ```
    ///
    /// *Incorrect* usage:
    ///
    /// ```rust,no_run
    /// use std::ptr::NonNull;
    ///
    /// let ptr1 = NonNull::new(Box::into_raw(Box::new(0u8))).unwrap();
    /// let ptr2 = NonNull::new(Box::into_raw(Box::new(1u8))).unwrap();
    /// let diff = (ptr2.addr().get() as isize).wrapping_sub(ptr1.addr().get() as isize);
    /// // Make ptr2_other an "alias" of ptr2.add(1), but derived from ptr1.
    /// let diff_plus_1 = diff.wrapping_add(1);
    /// let ptr2_other = NonNull::new(ptr1.as_ptr().wrapping_byte_offset(diff_plus_1)).unwrap();
    /// assert_eq!(ptr2.addr(), ptr2_other.addr());
    /// // Since ptr2_other and ptr2 are derived from pointers to different objects,
    /// // computing their offset is undefined behavior, even though
    /// // they point to addresses that are in-bounds of the same object!
    ///
    /// let one = unsafe { ptr2_other.offset_from(ptr2) }; // Undefined Behavior! ⚠️
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn offset_from(self, origin: ConstNonNull<T>) -> isize
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset_from`.
        unsafe { self.as_ptr().offset_from(origin.as_ptr()) }
    }

    /// Calculates the distance between two pointers within the same allocation. The returned value is in
    /// units of **bytes**.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [`offset_from`][NonNull::offset_from] on it. See that method for
    /// documentation and safety requirements.
    ///
    /// For non-`Sized` pointees this operation considers only the data pointers,
    /// ignoring the metadata.
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_offset_from<U: ?Sized>(self, origin: ConstNonNull<U>) -> isize {
        // SAFETY: the caller must uphold the safety contract for `byte_offset_from`.
        unsafe { self.as_ptr().byte_offset_from(origin.as_ptr()) }
    }

    // N.B. `wrapping_offset``, `wrapping_add`, etc are not implemented because they can wrap to null

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// See [`ptr::read`] for safety concerns and examples.
    ///
    /// [`ptr::read`]: core::ptr::read()
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn read(self) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `read`.
        unsafe { ptr::read(self.as_ptr()) }
    }

    /// Performs a volatile read of the value from `self` without moving it. This
    /// leaves the memory in `self` unchanged.
    ///
    /// Volatile operations are intended to act on I/O memory, and are guaranteed
    /// to not be elided or reordered by the compiler across other volatile
    /// operations.
    ///
    /// See [`ptr::read_volatile`] for safety concerns and examples.
    ///
    /// [`ptr::read_volatile`]: core::ptr::read_volatile()
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn read_volatile(self) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `read_volatile`.
        unsafe { ptr::read_volatile(self.as_ptr()) }
    }

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// Unlike `read`, the pointer may be unaligned.
    ///
    /// See [`ptr::read_unaligned`] for safety concerns and examples.
    ///
    /// [`ptr::read_unaligned`]: core::ptr::read_unaligned()
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn read_unaligned(self) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `read_unaligned`.
        unsafe { ptr::read_unaligned(self.as_ptr()) }
    }

    /// Copies `count * size_of<T>` bytes from `self` to `dest`. The source
    /// and destination may overlap.
    ///
    /// NOTE: this has the *same* argument order as [`ptr::copy`].
    ///
    /// See [`ptr::copy`] for safety concerns and examples.
    ///
    /// [`ptr::copy`]: core::ptr::copy()
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn copy_to(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `copy`.
        unsafe { ptr::copy(self.as_ptr(), dest.as_ptr(), count) }
    }

    /// Copies `count * size_of<T>` bytes from `self` to `dest`. The source
    /// and destination may *not* overlap.
    ///
    /// NOTE: this has the *same* argument order as [`ptr::copy_nonoverlapping`].
    ///
    /// See [`ptr::copy_nonoverlapping`] for safety concerns and examples.
    ///
    /// [`ptr::copy_nonoverlapping`]: core::ptr::copy_nonoverlapping()
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn copy_to_nonoverlapping(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `copy_nonoverlapping`.
        unsafe { ptr::copy_nonoverlapping(self.as_ptr(), dest.as_ptr(), count) }
    }

    /// Computes the offset that needs to be applied to the pointer in order to make it aligned to
    /// `align`.
    ///
    /// If it is not possible to align the pointer, the implementation returns
    /// `usize::MAX`.
    ///
    /// The offset is expressed in number of `T` elements, and not bytes.
    ///
    /// There are no guarantees whatsoever that offsetting the pointer will not overflow or go
    /// beyond the allocation that the pointer points into. It is up to the caller to ensure that
    /// the returned offset is correct in all terms other than alignment.
    ///
    /// When this is called during compile-time evaluation (which is unstable), the implementation
    /// may return `usize::MAX` in cases where that can never happen at runtime. This is because the
    /// actual alignment of pointers is not known yet during compile-time, so an offset with
    /// guaranteed alignment can sometimes not be computed. For example, a buffer declared as `[u8;
    /// N]` might be allocated at an odd or an even address, but at compile-time this is not yet
    /// known, so the execution has to be correct for either choice. It is therefore impossible to
    /// find an offset that is guaranteed to be 2-aligned. (This behavior is subject to change, as usual
    /// for unstable APIs.)
    ///
    /// # Panics
    ///
    /// The function panics if `align` is not a power-of-two.
    ///
    /// # Examples
    ///
    /// Accessing adjacent `u8` as `u16`
    ///
    /// ```
    /// use std::mem::align_of;
    /// use std::ptr::NonNull;
    ///
    /// # unsafe {
    /// let x = [5_u8, 6, 7, 8, 9];
    /// let ptr = NonNull::new(x.as_ptr() as *mut u8).unwrap();
    /// let offset = ptr.align_offset(align_of::<u16>());
    ///
    /// if offset < x.len() - 1 {
    ///     let u16_ptr = ptr.add(offset).cast::<u16>();
    ///     assert!(u16_ptr.read() == u16::from_ne_bytes([5, 6]) || u16_ptr.read() == u16::from_ne_bytes([6, 7]));
    /// } else {
    ///     // while the pointer can be aligned via `offset`, it would point
    ///     // outside the allocation
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn align_offset(self, align: usize) -> usize
    where
        T: Sized,
    {
        self.pointer.align_offset(align)
    }

    /// Returns whether the pointer is properly aligned for `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// // On some platforms, the alignment of i32 is less than 4.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    ///
    /// let data = AlignedI32(42);
    /// let ptr = NonNull::<AlignedI32>::from(&data);
    ///
    /// assert!(ptr.is_aligned());
    /// assert!(!NonNull::new(ptr.as_ptr().wrapping_byte_add(1)).unwrap().is_aligned());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_aligned(self) -> bool
    where
        T: Sized,
    {
        self.as_ptr().is_aligned()
    }
}

impl<T> ConstNonNull<[T]> {
    /// Creates a non-null raw slice from a thin pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, but dereferencing the return value is unsafe.
    /// See the documentation of [`slice::from_raw_parts`](core::slice::from_raw_parts) for slice safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::ptr::NonNull;
    ///
    /// // create a slice pointer when starting out with a pointer to the first element
    /// let mut x = [5, 6, 7];
    /// let nonnull_pointer = NonNull::new(x.as_mut_ptr()).unwrap();
    /// let slice = NonNull::slice_from_raw_parts(nonnull_pointer, 3);
    /// assert_eq!(unsafe { slice.as_ref()[2] }, 7);
    /// ```
    ///
    /// (Note that this example artificially demonstrates a use of this method,
    /// but `let slice = NonNull::from(&x[..]);` would be a better way to write code like this.)
    #[must_use]
    #[inline]
    pub const fn slice_from_raw_parts(data: ConstNonNull<T>, len: usize) -> Self {
        // SAFETY: `data` is a `NonNull` pointer which is necessarily non-null
        Self { pointer: NonNull::slice_from_raw_parts(data.pointer, len) }
    }

    /// Returns the length of a non-null raw slice.
    ///
    /// The returned value is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, even when the non-null raw slice cannot be dereferenced to a slice
    /// because the pointer does not have a valid address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::ptr::NonNull;
    ///
    /// let slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(), 3);
    /// assert_eq!(slice.len(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub const fn len(self) -> usize {
        self.as_ptr().len()
    }

    /// Returns `true` if the non-null raw slice has a length of 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::ptr::NonNull;
    ///
    /// let slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(), 3);
    /// assert!(!slice.is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns a non-null pointer to the slice's buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(slice_ptr_get)]
    /// use std::ptr::NonNull;
    ///
    /// let slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(), 3);
    /// assert_eq!(slice.as_non_null_ptr(), NonNull::<i8>::dangling());
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_non_null_ptr(self) -> ConstNonNull<T> {
        self.cast()
    }
}

impl<T: ?Sized> Clone for ConstNonNull<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for ConstNonNull<T> {}

impl<T: ?Sized> fmt::Debug for ConstNonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> fmt::Pointer for ConstNonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> Eq for ConstNonNull<T> {}

impl<T: ?Sized> PartialEq for ConstNonNull<T> {
    #[inline]
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl<T: ?Sized> Ord for ConstNonNull<T> {
    #[inline]
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T: ?Sized> PartialOrd for ConstNonNull<T> {
    #[inline]
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

impl<T: ?Sized> hash::Hash for ConstNonNull<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl<T: ?Sized> From<Unique<T>> for ConstNonNull<T> {
    #[inline]
    fn from(unique: Unique<T>) -> Self {
        Self { pointer: unique.as_non_null_ptr() }
    }
}

impl<T: ?Sized> From<&T> for ConstNonNull<T> {
    /// Converts a `&T` to a `NonNull<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(r: &T) -> Self {
        ConstNonNull::from_ref(r)
    }
}
