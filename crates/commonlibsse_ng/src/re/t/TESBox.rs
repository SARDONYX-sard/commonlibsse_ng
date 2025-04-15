//! The `Box<T>` type for heap allocation.
//!
//! [`Box<T>`], casually referred to as a 'box', provides the simplest form of
//! heap allocation in Rust. Boxes provide ownership for this allocation, and
//! drop their contents when they go out of scope. Boxes also ensure that they
//! never allocate more than `isize::MAX` bytes.
//!
//! # Examples
//!
//! Move a value from the stack to the heap by creating a [`Box`]:
//!
//! ```
//! let val: u8 = 5;
//! let boxed: Box<u8> = Box::new(val);
//! ```
//!
//! Move a value from a [`Box`] back to the stack by [dereferencing]:
//!
//! ```
//! let boxed: Box<u8> = Box::new(5);
//! let val: u8 = *boxed;
//! ```
//!
//! Creating a recursive data structure:
//!
//! ```
//! # #[allow(dead_code)]
//! #[derive(Debug)]
//! enum List<T> {
//!     Cons(T, Box<List<T>>),
//!     Nil,
//! }
//!
//! let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
//! println!("{list:?}");
//! ```
//!
//! This will print `Cons(1, Cons(2, Nil))`.
//!
//! Recursive structures must be boxed, because if the definition of `Cons`
//! looked like this:
//!
//! ```compile_fail,E0072
//! # enum List<T> {
//! Cons(T, List<T>),
//! # }
//! ```
//!
//! It wouldn't work. This is because the size of a `List` depends on how many
//! elements are in the list, and so we don't know how much memory to allocate
//! for a `Cons`. By introducing a [`Box<T>`], which has a defined size, we know how
//! big `Cons` needs to be.
//!
//! # Memory layout
//!
//! For non-zero-sized values, a [`Box`] will use the [`Global`] allocator for its allocation. It is
//! valid to convert both ways between a [`Box`] and a raw pointer allocated with the [`Global`]
//! allocator, given that the [`Layout`] used with the allocator is correct for the type and the raw
//! pointer points to a valid value of the right type. More precisely, a `value: *mut T` that has
//! been allocated with the [`Global`] allocator with `Layout::for_value(&*value)` may be converted
//! into a box using [`Box::<T>::from_raw(value)`]. Conversely, the memory backing a `value: *mut T`
//! obtained from [`Box::<T>::into_raw`] may be deallocated using the [`Global`] allocator with
//! [`Layout::for_value(&*value)`].
//!
//! For zero-sized values, the `Box` pointer has to be non-null and sufficiently aligned. The
//! recommended way to build a Box to a ZST if `Box::new` cannot be used is to use
//! [`ptr::NonNull::dangling`].
//!
//! On top of these basic layout requirements, a `Box<T>` must point to a valid value of `T`.
//!
//! So long as `T: Sized`, a `Box<T>` is guaranteed to be represented
//! as a single pointer and is also ABI-compatible with C pointers
//! (i.e. the C type `T*`). This means that if you have extern "C"
//! Rust functions that will be called from C, you can define those
//! Rust functions using `Box<T>` types, and use `T*` as corresponding
//! type on the C side. As an example, consider this C header which
//! declares functions that create and destroy some kind of `Foo`
//! value:
//!
//! ```c
//! /* C header */
//!
//! /* Returns ownership to the caller */
//! struct Foo* foo_new(void);
//!
//! /* Takes ownership from the caller; no-op when invoked with null */
//! void foo_delete(struct Foo*);
//! ```
//!
//! These two functions might be implemented in Rust as follows. Here, the
//! `struct Foo*` type from C is translated to `Box<Foo>`, which captures
//! the ownership constraints. Note also that the nullable argument to
//! `foo_delete` is represented in Rust as `Option<Box<Foo>>`, since `Box<Foo>`
//! cannot be null.
//!
//! ```
//! #[repr(C)]
//! pub struct Foo;
//!
//! #[unsafe(no_mangle)]
//! pub extern "C" fn foo_new() -> Box<Foo> {
//!     Box::new(Foo)
//! }
//!
//! #[unsafe(no_mangle)]
//! pub extern "C" fn foo_delete(_: Option<Box<Foo>>) {}
//! ```
//!
//! Even though `Box<T>` has the same representation and C ABI as a C pointer,
//! this does not mean that you can convert an arbitrary `T*` into a `Box<T>`
//! and expect things to work. `Box<T>` values will always be fully aligned,
//! non-null pointers. Moreover, the destructor for `Box<T>` will attempt to
//! free the value with the global allocator. In general, the best practice
//! is to only use `Box<T>` for pointers that originated from the global
//! allocator.
//!
//! **Important.** At least at present, you should avoid using
//! `Box<T>` types for functions that are defined in C but invoked
//! from Rust. In those cases, you should directly mirror the C types
//! as closely as possible. Using types like `Box<T>` where the C
//! definition is just using `T*` can lead to undefined behavior, as
//! described in [rust-lang/unsafe-code-guidelines#198][ucg#198].
//!
//! # Considerations for unsafe code
//!
//! **Warning: This section is not normative and is subject to change, possibly
//! being relaxed in the future! It is a simplified summary of the rules
//! currently implemented in the compiler.**
//!
//! The aliasing rules for `Box<T>` are the same as for `&mut T`. `Box<T>`
//! asserts uniqueness over its content. Using raw pointers derived from a box
//! after that box has been mutated through, moved or borrowed as `&mut T`
//! is not allowed. For more guidance on working with box from unsafe code, see
//! [rust-lang/unsafe-code-guidelines#326][ucg#326].
//!
//! # Editions
//!
//! A special case exists for the implementation of `IntoIterator` for arrays on the Rust 2021
//! edition, as documented [here][array]. Unfortunately, it was later found that a similar
//! workaround should be added for boxed slices, and this was applied in the 2024 edition.
//!
//! Specifically, `IntoIterator` is implemented for `Box<[T]>` on all editions, but specific calls
//! to `into_iter()` for boxed slices will defer to the slice implementation on editions before
//! 2024:
//!
//! ```rust,edition2021
//! // Rust 2015, 2018, and 2021:
//!
//! # #![allow(boxed_slice_into_iter)] // override our `deny(warnings)`
//! let boxed_slice: Box<[i32]> = vec![0; 3].into_boxed_slice();
//!
//! // This creates a slice iterator, producing references to each value.
//! for item in boxed_slice.into_iter().enumerate() {
//!     let (i, x): (usize, &i32) = item;
//!     println!("boxed_slice[{i}] = {x}");
//! }
//!
//! // The `boxed_slice_into_iter` lint suggests this change for future compatibility:
//! for item in boxed_slice.iter().enumerate() {
//!     let (i, x): (usize, &i32) = item;
//!     println!("boxed_slice[{i}] = {x}");
//! }
//!
//! // You can explicitly iterate a boxed slice by value using `IntoIterator::into_iter`
//! for item in IntoIterator::into_iter(boxed_slice).enumerate() {
//!     let (i, x): (usize, i32) = item;
//!     println!("boxed_slice[{i}] = {x}");
//! }
//! ```
//!
//! Similar to the array implementation, this may be modified in the future to remove this override,
//! and it's best to avoid relying on this edition-dependent behavior if you wish to preserve
//! compatibility with future versions of the compiler.
//!
//! [ucg#198]: https://github.com/rust-lang/unsafe-code-guidelines/issues/198
//! [ucg#326]: https://github.com/rust-lang/unsafe-code-guidelines/issues/326
//! [dereferencing]: core::ops::Deref
//! [`Box::<T>::from_raw(value)`]: Box::from_raw
//! [`Global`]: crate::alloc::Global
//! [`Layout`]: crate::alloc::Layout
//! [`Layout::for_value(&*value)`]: crate::alloc::Layout::for_value
//! [valid]: ptr#safety
#![allow(clippy::explicit_auto_deref)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::use_self)]

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::error::Error;
use core::hash::{Hash, Hasher};
use core::mem::{self};
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use core::{fmt, ptr};
use std::alloc::handle_alloc_error;

use crate::re::MemoryManager::TESGlobalAlloc as Global;
use crate::rex::stdx::alloc::{AllocError, Allocator};
use core::alloc::Layout;
use unique::Unique;

// The declaration of the `Box` struct must be kept in sync with the
// compiler or ICEs will happen.

/// Heap memory allocation smart pointer that `MemoryManager` `malloc` and `free` trigger by default.
///
/// The allocator side is always of type zero size and represents `owned *mut ptr`. (i.e. size is type ptr)
// #[repr(transparent)] // FIXME: maybe need this.
pub struct TESBox<T: ?Sized, A: Allocator = Global>(Unique<T>, A);
const _: () = assert!(core::mem::size_of::<TESBox<()>>() == core::mem::size_of::<usize>());

// #[repr(transparent)]
// pub struct TESBox<T: ?Sized, A: Allocator = Global>(Unique<T>, core::marker::PhantomData<A>);
// const _: () = assert!(core::mem::size_of::<TESBox<()>>() == core::mem::size_of::<usize>());

impl<T> TESBox<T> {
    /// Allocates memory on the heap and then places `x` into it.
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = Box::new(5);
    /// ```
    #[inline(always)]
    #[must_use]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn new(x: T) -> Self {
        Self::new_in(x, Global)
    }

    /// Constructs a new box with uninitialized contents.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut five = Box::<u32>::new_uninit();
    /// // Deferred initialization:
    /// five.write(5);
    /// let five = unsafe { five.assume_init() };
    ///
    /// assert_eq!(*five, 5)
    /// ```
    #[must_use]
    #[inline]
    pub fn new_uninit() -> TESBox<mem::MaybeUninit<T>> {
        Self::new_uninit_in(Global)
    }

    /// Allocates memory on the heap then places `x` into it,
    /// returning an error if the allocation fails
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// let five = Box::try_new(5)?;
    /// # Ok::<(), std::alloc::AllocError>(())
    /// ```
    #[inline]
    pub fn try_new(x: T) -> Result<Self, AllocError> {
        Self::try_new_in(x, Global)
    }
}

impl<T, A: Allocator> TESBox<T, A> {
    /// Allocates memory in the given allocator then places `x` into it.
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::System;
    ///
    /// let five = Box::new_in(5, System);
    /// ```
    #[must_use]
    #[inline]
    pub fn new_in(x: T, alloc: A) -> Self
    where
        A: Allocator,
    {
        let mut boxed = Self::new_uninit_in(alloc);
        boxed.write(x);
        unsafe { boxed.assume_init() }
    }

    /// Allocates memory in the given allocator then places `x` into it,
    /// returning an error if the allocation fails
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::System;
    ///
    /// let five = Box::try_new_in(5, System)?;
    /// # Ok::<(), std::alloc::AllocError>(())
    /// ```
    #[inline]
    pub fn try_new_in(x: T, alloc: A) -> Result<Self, AllocError>
    where
        A: Allocator,
    {
        let mut boxed = Self::try_new_uninit_in(alloc)?;
        boxed.write(x);
        unsafe { Ok(boxed.assume_init()) }
    }

    /// Constructs a new box with uninitialized contents in the provided allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::System;
    ///
    /// let mut five = Box::<u32, _>::new_uninit_in(System);
    /// // Deferred initialization:
    /// five.write(5);
    /// let five = unsafe { five.assume_init() };
    ///
    /// assert_eq!(*five, 5)
    /// ```
    #[must_use]
    pub fn new_uninit_in(alloc: A) -> TESBox<mem::MaybeUninit<T>, A>
    where
        A: Allocator,
    {
        let layout = Layout::new::<mem::MaybeUninit<T>>();
        // NOTE: Prefer match over unwrap_or_else since closure sometimes not inline-able.
        // That would make code size bigger.
        match Self::try_new_uninit_in(alloc) {
            Ok(m) => m,
            Err(_) => handle_alloc_error(layout),
        }
    }

    /// Constructs a new box with uninitialized contents in the provided allocator,
    /// returning an error if the allocation fails
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::System;
    ///
    /// let mut five = Box::<u32, _>::try_new_uninit_in(System)?;
    /// // Deferred initialization:
    /// five.write(5);
    /// let five = unsafe { five.assume_init() };
    ///
    /// assert_eq!(*five, 5);
    /// # Ok::<(), std::alloc::AllocError>(())
    /// ```
    pub fn try_new_uninit_in(alloc: A) -> Result<TESBox<mem::MaybeUninit<T>, A>, AllocError>
    where
        A: Allocator,
    {
        let ptr = if core::mem::size_of::<T>() == 0 {
            NonNull::dangling()
        } else {
            let layout = Layout::new::<mem::MaybeUninit<T>>();
            alloc.allocate(layout)?.cast()
        };
        unsafe { Ok(TESBox::from_raw_in(ptr.as_ptr(), alloc)) }
    }
}

impl<T> TESBox<[T]> {
    /// Converts the boxed slice into a boxed array.
    ///
    /// This operation does not reallocate; the underlying array of the slice is simply reinterpreted as an array type.
    ///
    /// If `N` is not exactly equal to the length of `self`, then this method returns `None`.
    #[inline]
    #[must_use]
    pub fn into_array<const N: usize>(self) -> Option<TESBox<[T; N]>> {
        if self.len() == N {
            let ptr = Self::into_raw(self).cast::<[T; N]>();

            // SAFETY: The underlying array of a slice has the exact same layout as an actual array `[T; N]` if `N` is equal to the slice's length.
            let me = unsafe { TESBox::from_raw(ptr) };
            Some(me)
        } else {
            None
        }
    }
}

impl<T, A: Allocator> TESBox<mem::MaybeUninit<T>, A> {
    /// Converts to `Box<T, A>`.
    ///
    /// # Safety
    ///
    /// As with [`MaybeUninit::assume_init`],
    /// it is up to the caller to guarantee that the value
    /// really is in an initialized state.
    /// Calling this when the content is not yet fully initialized
    /// causes immediate undefined behavior.
    ///
    /// [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init
    ///
    /// # Examples
    ///
    /// ```
    /// let mut five = Box::<u32>::new_uninit();
    /// // Deferred initialization:
    /// five.write(5);
    /// let five: Box<u32> = unsafe { five.assume_init() };
    ///
    /// assert_eq!(*five, 5)
    /// ```
    #[inline]
    pub unsafe fn assume_init(self) -> TESBox<T, A> {
        let (raw, alloc) = TESBox::into_raw_with_allocator(self);
        unsafe { TESBox::from_raw_in(raw as *mut T, alloc) }
    }

    /// Writes the value and converts to `Box<T, A>`.
    ///
    /// This method converts the box similarly to [`Box::assume_init`] but
    /// writes `value` into it before conversion thus guaranteeing safety.
    /// In some scenarios use of this method may improve performance because
    /// the compiler may be able to optimize copying from stack.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(box_uninit_write)]
    ///
    /// let big_box = Box::<[usize; 1024]>::new_uninit();
    ///
    /// let mut array = [0; 1024];
    /// for (i, place) in array.iter_mut().enumerate() {
    ///     *place = i;
    /// }
    ///
    /// // The optimizer may be able to elide this copy, so previous code writes
    /// // to heap directly.
    /// let big_box = Box::write(big_box, array);
    ///
    /// for (i, x) in big_box.iter().enumerate() {
    ///     assert_eq!(*x, i);
    /// }
    /// ```
    #[inline]
    pub fn write(mut boxed: Self, value: T) -> TESBox<T, A> {
        unsafe {
            (*boxed).write(value);
            boxed.assume_init()
        }
    }
}

impl<T, A: Allocator> TESBox<[mem::MaybeUninit<T>], A> {
    /// Converts to `Box<[T], A>`.
    ///
    /// # Safety
    ///
    /// As with [`MaybeUninit::assume_init`],
    /// it is up to the caller to guarantee that the values
    /// really are in an initialized state.
    /// Calling this when the content is not yet fully initialized
    /// causes immediate undefined behavior.
    ///
    /// [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init
    ///
    /// # Examples
    ///
    /// ```
    /// let mut values = Box::<[u32]>::new_uninit_slice(3);
    /// // Deferred initialization:
    /// values[0].write(1);
    /// values[1].write(2);
    /// values[2].write(3);
    /// let values = unsafe { values.assume_init() };
    ///
    /// assert_eq!(*values, [1, 2, 3])
    /// ```
    #[inline]
    pub unsafe fn assume_init(self) -> TESBox<[T], A> {
        let (raw, alloc) = TESBox::into_raw_with_allocator(self);
        unsafe { TESBox::from_raw_in(raw as *mut [T], alloc) }
    }
}

impl<T: ?Sized> TESBox<T> {
    /// Constructs a box from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    ///
    /// The raw pointer must point to a block of memory allocated by the global allocator.
    ///
    /// The safety conditions are described in the [memory layout] section.
    ///
    /// # Examples
    ///
    /// Recreate a `Box` which was previously converted to a raw pointer
    /// using [`Box::into_raw`]:
    /// ```
    /// let x = Box::new(5);
    /// let ptr = Box::into_raw(x);
    /// let x = unsafe { Box::from_raw(ptr) };
    /// ```
    /// Manually create a `Box` from scratch by using the global allocator:
    /// ```
    /// use std::alloc::{alloc, Layout};
    ///
    /// unsafe {
    ///     let ptr = alloc(Layout::new::<i32>()) as *mut i32;
    ///     // In general .write is required to avoid attempting to destruct
    ///     // the (uninitialized) previous contents of `ptr`, though for this
    ///     // simple example `*ptr = 5` would have worked as well.
    ///     ptr.write(5);
    ///     let x = Box::from_raw(ptr);
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    /// [`Layout`]: crate::Layout
    #[inline]
    #[must_use = "call `drop(Box::from_raw(ptr))` if you intend to drop the `Box`"]
    pub const unsafe fn from_raw(raw: *mut T) -> Self {
        unsafe { Self::from_raw_in(raw, Global) }
    }

    /// Constructs a box from a `NonNull` pointer.
    ///
    /// After calling this function, the `NonNull` pointer is owned by
    /// the resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same `NonNull` pointer.
    ///
    /// The non-null pointer must point to a block of memory allocated by the global allocator.
    ///
    /// The safety conditions are described in the [memory layout] section.
    ///
    /// # Examples
    ///
    /// Recreate a `Box` which was previously converted to a `NonNull`
    /// pointer using [`Box::into_non_null`]:
    /// ```
    /// #![feature(box_vec_non_null)]
    ///
    /// let x = Box::new(5);
    /// let non_null = Box::into_non_null(x);
    /// let x = unsafe { Box::from_non_null(non_null) };
    /// ```
    /// Manually create a `Box` from scratch by using the global allocator:
    /// ```
    /// #![feature(box_vec_non_null)]
    ///
    /// use std::alloc::{alloc, Layout};
    /// use std::ptr::NonNull;
    ///
    /// unsafe {
    ///     let non_null = NonNull::new(alloc(Layout::new::<i32>()).cast::<i32>())
    ///         .expect("allocation failed");
    ///     // In general .write is required to avoid attempting to destruct
    ///     // the (uninitialized) previous contents of `non_null`.
    ///     non_null.write(5);
    ///     let x = Box::from_non_null(non_null);
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    /// [`Layout`]: crate::Layout
    #[inline]
    #[must_use = "call `drop(Box::from_non_null(ptr))` if you intend to drop the `Box`"]
    pub unsafe fn from_non_null(ptr: NonNull<T>) -> Self {
        unsafe { Self::from_raw(ptr.as_ptr()) }
    }
}

impl<T: ?Sized, A: Allocator> TESBox<T, A> {
    /// Constructs a box from a raw pointer in the given allocator.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    ///
    /// The raw pointer must point to a block of memory allocated by `alloc`.
    ///
    /// # Examples
    ///
    /// Recreate a `Box` which was previously converted to a raw pointer
    /// using [`Box::into_raw_with_allocator`]:
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::System;
    ///
    /// let x = Box::new_in(5, System);
    /// let (ptr, alloc) = Box::into_raw_with_allocator(x);
    /// let x = unsafe { Box::from_raw_in(ptr, alloc) };
    /// ```
    /// Manually create a `Box` from scratch by using the system allocator:
    /// ```
    /// #![feature(allocator_api, slice_ptr_get)]
    ///
    /// use std::alloc::{Allocator, Layout, System};
    ///
    /// unsafe {
    ///     let ptr = System.allocate(Layout::new::<i32>())?.as_mut_ptr() as *mut i32;
    ///     // In general .write is required to avoid attempting to destruct
    ///     // the (uninitialized) previous contents of `ptr`, though for this
    ///     // simple example `*ptr = 5` would have worked as well.
    ///     ptr.write(5);
    ///     let x = Box::from_raw_in(ptr, System);
    /// }
    /// # Ok::<(), std::alloc::AllocError>(())
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    /// [`Layout`]: crate::Layout
    #[inline]
    pub const unsafe fn from_raw_in(raw: *mut T, alloc: A) -> Self {
        TESBox(unsafe { Unique::new_unchecked(raw) }, alloc)
    }

    /// Constructs a box from a `NonNull` pointer in the given allocator.
    ///
    /// After calling this function, the `NonNull` pointer is owned by
    /// the resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    ///
    /// The non-null pointer must point to a block of memory allocated by `alloc`.
    ///
    /// # Examples
    ///
    /// Recreate a `Box` which was previously converted to a `NonNull` pointer
    /// using [`Box::into_non_null_with_allocator`]:
    /// ```
    /// #![feature(allocator_api, box_vec_non_null)]
    ///
    /// use std::alloc::System;
    ///
    /// let x = Box::new_in(5, System);
    /// let (non_null, alloc) = Box::into_non_null_with_allocator(x);
    /// let x = unsafe { Box::from_non_null_in(non_null, alloc) };
    /// ```
    /// Manually create a `Box` from scratch by using the system allocator:
    /// ```
    /// #![feature(allocator_api, box_vec_non_null, slice_ptr_get)]
    ///
    /// use std::alloc::{Allocator, Layout, System};
    ///
    /// unsafe {
    ///     let non_null = System.allocate(Layout::new::<i32>())?.cast::<i32>();
    ///     // In general .write is required to avoid attempting to destruct
    ///     // the (uninitialized) previous contents of `non_null`.
    ///     non_null.write(5);
    ///     let x = Box::from_non_null_in(non_null, System);
    /// }
    /// # Ok::<(), std::alloc::AllocError>(())
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    /// [`Layout`]: crate::Layout
    #[inline]
    pub const unsafe fn from_non_null_in(raw: NonNull<T>, alloc: A) -> Self {
        // SAFETY: guaranteed by the caller.
        unsafe { TESBox::from_raw_in(raw.as_ptr(), alloc) }
    }

    /// Consumes the `Box`, returning a wrapped raw pointer.
    ///
    /// The pointer will be properly aligned and non-null.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the raw pointer back into a `Box` with the
    /// [`Box::from_raw`] function, allowing the `Box` destructor to perform
    /// the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_raw(b)` instead of `b.into_raw()`. This
    /// is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    /// Converting the raw pointer back into a `Box` with [`Box::from_raw`]
    /// for automatic cleanup:
    /// ```
    /// let x = Box::new(String::from("Hello"));
    /// let ptr = Box::into_raw(x);
    /// let x = unsafe { Box::from_raw(ptr) };
    /// ```
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// use std::alloc::{dealloc, Layout};
    /// use std::ptr;
    ///
    /// let x = Box::new(String::from("Hello"));
    /// let ptr = Box::into_raw(x);
    /// unsafe {
    ///     ptr::drop_in_place(ptr);
    ///     dealloc(ptr as *mut u8, Layout::new::<String>());
    /// }
    /// ```
    /// Note: This is equivalent to the following:
    /// ```
    /// let x = Box::new(String::from("Hello"));
    /// let ptr = Box::into_raw(x);
    /// unsafe {
    ///     drop(Box::from_raw(ptr));
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    #[must_use = "losing the pointer will leak memory"]
    #[inline]
    pub fn into_raw(b: Self) -> *mut T {
        // Make sure Miri realizes that we transition from a noalias pointer to a raw pointer here.
        &raw mut *Self::into_raw_with_allocator(b).0
    }

    /// Consumes the `Box`, returning a wrapped `NonNull` pointer.
    ///
    /// The pointer will be properly aligned.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the `NonNull` pointer back into a `Box` with the
    /// [`Box::from_non_null`] function, allowing the `Box` destructor to
    /// perform the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_non_null(b)` instead of `b.into_non_null()`.
    /// This is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    /// Converting the `NonNull` pointer back into a `Box` with [`Box::from_non_null`]
    /// for automatic cleanup:
    /// ```
    /// #![feature(box_vec_non_null)]
    ///
    /// let x = Box::new(String::from("Hello"));
    /// let non_null = Box::into_non_null(x);
    /// let x = unsafe { Box::from_non_null(non_null) };
    /// ```
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// #![feature(box_vec_non_null)]
    ///
    /// use std::alloc::{dealloc, Layout};
    ///
    /// let x = Box::new(String::from("Hello"));
    /// let non_null = Box::into_non_null(x);
    /// unsafe {
    ///     non_null.drop_in_place();
    ///     dealloc(non_null.as_ptr().cast::<u8>(), Layout::new::<String>());
    /// }
    /// ```
    /// Note: This is equivalent to the following:
    /// ```
    /// #![feature(box_vec_non_null)]
    ///
    /// let x = Box::new(String::from("Hello"));
    /// let non_null = Box::into_non_null(x);
    /// unsafe {
    ///     drop(Box::from_non_null(non_null));
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    #[must_use = "losing the pointer will leak memory"]
    #[inline]
    pub fn into_non_null(b: Self) -> NonNull<T> {
        // SAFETY: `Box` is guaranteed to be non-null.
        unsafe { NonNull::new_unchecked(Self::into_raw(b)) }
    }

    /// Consumes the `Box`, returning a wrapped raw pointer and the allocator.
    ///
    /// The pointer will be properly aligned and non-null.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the raw pointer back into a `Box` with the
    /// [`Box::from_raw_in`] function, allowing the `Box` destructor to perform
    /// the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_raw_with_allocator(b)` instead of `b.into_raw_with_allocator()`. This
    /// is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    /// Converting the raw pointer back into a `Box` with [`Box::from_raw_in`]
    /// for automatic cleanup:
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::System;
    ///
    /// let x = Box::new_in(String::from("Hello"), System);
    /// let (ptr, alloc) = Box::into_raw_with_allocator(x);
    /// let x = unsafe { Box::from_raw_in(ptr, alloc) };
    /// ```
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// #![feature(allocator_api)]
    ///
    /// use std::alloc::{Allocator, Layout, System};
    /// use std::ptr::{self, NonNull};
    ///
    /// let x = Box::new_in(String::from("Hello"), System);
    /// let (ptr, alloc) = Box::into_raw_with_allocator(x);
    /// unsafe {
    ///     ptr::drop_in_place(ptr);
    ///     let non_null = NonNull::new_unchecked(ptr);
    ///     alloc.deallocate(non_null.cast(), Layout::new::<String>());
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    #[must_use = "losing the pointer will leak memory"]
    #[inline]
    pub fn into_raw_with_allocator(b: Self) -> (*mut T, A) {
        let mut b = mem::ManuallyDrop::new(b);
        // We carefully get the raw pointer out in a way that Miri's aliasing model understands what
        // is happening: using the primitive "deref" of `Box`. In case `A` is *not* `Global`, we
        // want *no* aliasing requirements here!
        // In case `A` *is* `Global`, this does not quite have the right behavior; `into_raw`
        // works around that.
        let ptr = &raw mut **b;
        let alloc = unsafe { ptr::read(&b.1) };
        (ptr, alloc)
    }

    /// Consumes the `Box`, returning a wrapped `NonNull` pointer and the allocator.
    ///
    /// The pointer will be properly aligned.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the `NonNull` pointer back into a `Box` with the
    /// [`Box::from_non_null_in`] function, allowing the `Box` destructor to
    /// perform the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_non_null_with_allocator(b)` instead of
    /// `b.into_non_null_with_allocator()`. This is so that there is no
    /// conflict with a method on the inner type.
    ///
    /// # Examples
    /// Converting the `NonNull` pointer back into a `Box` with
    /// [`Box::from_non_null_in`] for automatic cleanup:
    /// ```
    /// #![feature(allocator_api, box_vec_non_null)]
    ///
    /// use std::alloc::System;
    ///
    /// let x = Box::new_in(String::from("Hello"), System);
    /// let (non_null, alloc) = Box::into_non_null_with_allocator(x);
    /// let x = unsafe { Box::from_non_null_in(non_null, alloc) };
    /// ```
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// #![feature(allocator_api, box_vec_non_null)]
    ///
    /// use std::alloc::{Allocator, Layout, System};
    ///
    /// let x = Box::new_in(String::from("Hello"), System);
    /// let (non_null, alloc) = Box::into_non_null_with_allocator(x);
    /// unsafe {
    ///     non_null.drop_in_place();
    ///     alloc.deallocate(non_null.cast::<u8>(), Layout::new::<String>());
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    #[must_use = "losing the pointer will leak memory"]
    #[inline]
    pub fn into_non_null_with_allocator(b: Self) -> (NonNull<T>, A) {
        let (ptr, alloc) = TESBox::into_raw_with_allocator(b);
        // SAFETY: `Box` is guaranteed to be non-null.
        unsafe { (NonNull::new_unchecked(ptr), alloc) }
    }

    /// Returns a raw mutable pointer to the `Box`'s contents.
    ///
    /// The caller must ensure that the `Box` outlives the pointer this
    /// function returns, or else it will end up dangling.
    ///
    /// This method guarantees that for the purpose of the aliasing model, this method
    /// does not materialize a reference to the underlying memory, and thus the returned pointer
    /// will remain valid when mixed with other calls to [`as_ptr`] and [`as_mut_ptr`].
    /// Note that calling other methods that materialize references to the memory
    /// may still invalidate this pointer.
    /// See the example below for how this guarantee can be used.
    ///
    /// # Examples
    ///
    /// Due to the aliasing guarantee, the following code is legal:
    ///
    /// ```rust
    /// #![feature(box_as_ptr)]
    ///
    /// unsafe {
    ///     let mut b = Box::new(0);
    ///     let ptr1 = Box::as_mut_ptr(&mut b);
    ///     ptr1.write(1);
    ///     let ptr2 = Box::as_mut_ptr(&mut b);
    ///     ptr2.write(2);
    ///     // Notably, the write to `ptr2` did *not* invalidate `ptr1`:
    ///     ptr1.write(3);
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: Self::as_mut_ptr
    /// [`as_ptr`]: Self::as_ptr
    #[inline]
    pub fn as_mut_ptr(b: &mut Self) -> *mut T {
        // This is a primitive deref, not going through `DerefMut`, and therefore not materializing
        // any references.
        &raw mut **b
    }

    /// Returns a raw pointer to the `Box`'s contents.
    ///
    /// The caller must ensure that the `Box` outlives the pointer this
    /// function returns, or else it will end up dangling.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the `Box`, use [`as_mut_ptr`].
    ///
    /// This method guarantees that for the purpose of the aliasing model, this method
    /// does not materialize a reference to the underlying memory, and thus the returned pointer
    /// will remain valid when mixed with other calls to [`as_ptr`] and [`as_mut_ptr`].
    /// Note that calling other methods that materialize mutable references to the memory,
    /// as well as writing to this memory, may still invalidate this pointer.
    /// See the example below for how this guarantee can be used.
    ///
    /// # Examples
    ///
    /// Due to the aliasing guarantee, the following code is legal:
    ///
    /// ```rust
    /// #![feature(box_as_ptr)]
    ///
    /// unsafe {
    ///     let mut v = Box::new(0);
    ///     let ptr1 = Box::as_ptr(&v);
    ///     let ptr2 = Box::as_mut_ptr(&mut v);
    ///     let _val = ptr2.read();
    ///     // No write to this memory has happened yet, so `ptr1` is still valid.
    ///     let _val = ptr1.read();
    ///     // However, once we do a write...
    ///     ptr2.write(1);
    ///     // ... `ptr1` is no longer valid.
    ///     // This would be UB: let _val = ptr1.read();
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: Self::as_mut_ptr
    /// [`as_ptr`]: Self::as_ptr
    #[inline]
    pub fn as_ptr(b: &Self) -> *const T {
        // This is a primitive deref, not going through `DerefMut`, and therefore not materializing
        // any references.
        &raw const **b
    }

    /// Returns a reference to the underlying allocator.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::allocator(&b)` instead of `b.allocator()`. This
    /// is so that there is no conflict with a method on the inner type.
    #[inline]
    pub const fn allocator(b: &Self) -> &A {
        &b.1
    }

    /// Consumes and leaks the `Box`, returning a mutable reference,
    /// `&'a mut T`.
    ///
    /// Note that the type `T` must outlive the chosen lifetime `'a`. If the type
    /// has only static references, or none at all, then this may be chosen to be
    /// `'static`.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak. If this is not acceptable, the reference should first be wrapped
    /// with the [`Box::from_raw`] function producing a `Box`. This `Box` can
    /// then be dropped which will properly destroy `T` and release the
    /// allocated memory.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::leak(b)` instead of `b.leak()`. This
    /// is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    ///
    /// Simple usage:
    ///
    /// ```
    /// let x = Box::new(41);
    /// let static_ref: &'static mut usize = Box::leak(x);
    /// *static_ref += 1;
    /// assert_eq!(*static_ref, 42);
    /// # // FIXME(https://github.com/rust-lang/miri/issues/3670):
    /// # // use -Zmiri-disable-leak-check instead of unleaking in tests meant to leak.
    /// # drop(unsafe { Box::from_raw(static_ref) });
    /// ```
    ///
    /// Unsized data:
    ///
    /// ```
    /// let x = vec![1, 2, 3].into_boxed_slice();
    /// let static_ref = Box::leak(x);
    /// static_ref[0] = 4;
    /// assert_eq!(*static_ref, [4, 2, 3]);
    /// # // FIXME(https://github.com/rust-lang/miri/issues/3670):
    /// # // use -Zmiri-disable-leak-check instead of unleaking in tests meant to leak.
    /// # drop(unsafe { Box::from_raw(static_ref) });
    /// ```
    #[inline]
    pub fn leak<'a>(b: Self) -> &'a mut T
    where
        A: 'a,
    {
        unsafe { &mut *TESBox::into_raw(b) }
    }

    /// Converts a `Box<T>` into a `Pin<Box<T>>`. If `T` does not implement [`Unpin`], then
    /// `*boxed` will be pinned in memory and unable to be moved.
    ///
    /// This conversion does not allocate on the heap and happens in place.
    ///
    /// This is also available via [`From`].
    ///
    /// Constructing and pinning a `Box` with <code>Box::into_pin([Box::new]\(x))</code>
    /// can also be written more concisely using <code>[Box::pin]\(x)</code>.
    /// This `into_pin` method is useful if you already have a `Box<T>`, or you are
    /// constructing a (pinned) `Box` in a different way than with [`Box::new`].
    ///
    /// # Notes
    ///
    /// It's not recommended that crates add an impl like `From<Box<T>> for Pin<T>`,
    /// as it'll introduce an ambiguity when calling `Pin::from`.
    /// A demonstration of such a poor impl is shown below.
    ///
    /// ```compile_fail
    /// # use std::pin::Pin;
    /// struct Foo; // A type defined in this crate.
    /// impl From<Box<()>> for Pin<Foo> {
    ///     fn from(_: Box<()>) -> Pin<Foo> {
    ///         Pin::new(Foo)
    ///     }
    /// }
    ///
    /// let foo = Box::new(());
    /// let bar = Pin::from(foo);
    /// ```
    pub const fn into_pin(boxed: Self) -> Pin<Self>
    where
        A: 'static,
    {
        // It's not possible to move or replace the insides of a `Pin<Box<T>>`
        // when `T: !Unpin`, so it's safe to pin it directly without any
        // additional requirements.
        unsafe { Pin::new_unchecked(boxed) }
    }
}

impl<T: ?Sized, A: Allocator> Drop for TESBox<T, A> {
    #[inline]
    fn drop(&mut self) {
        // the T in the Box is dropped by the compiler before the destructor is run

        let ptr = self.0;

        unsafe {
            let layout = Layout::for_value::<T>(&**self);
            if layout.size() != 0 {
                self.1.deallocate(From::from(ptr.cast()), layout);
            }
        }
    }
}

impl<T: Default> Default for TESBox<T> {
    /// Creates a `Box<T>`, with the `Default` value for T.
    #[inline]
    fn default() -> Self {
        TESBox::write(TESBox::new_uninit(), T::default())
    }
}

impl<T> Default for TESBox<[T]> {
    #[inline]
    fn default() -> Self {
        // SAFETY: [T; 0] is a valid allocation
        let layout = Layout::array::<T>(0).unwrap();
        let Ok(ptr) = Global::allocate(&Global, layout) else {
            panic!("TESBox: allocation failed for empty slice");
        };

        let slice = unsafe { core::slice::from_raw_parts_mut(ptr.cast().as_ptr(), 0) };
        let slice = unsafe { NonNull::new_unchecked(slice as *mut [T]) };
        unsafe { Self::from_raw_in(slice.as_ptr(), Global) }
    }
}

impl<T: Clone, A: Allocator + Clone> Clone for TESBox<T, A> {
    /// Returns a new box with a `clone()` of this box's contents.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Box::new(5);
    /// let y = x.clone();
    ///
    /// // The value is the same
    /// assert_eq!(x, y);
    ///
    /// // But they are unique objects
    /// assert_ne!(&*x as *const i32, &*y as *const i32);
    /// ```
    #[inline]
    fn clone(&self) -> Self {
        let cloned = (**self).clone();
        TESBox::new_in(cloned, self.1.clone())
    }

    /// Copies `source`'s contents into `self` without creating a new allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Box::new(5);
    /// let mut y = Box::new(10);
    /// let yp: *const i32 = &*y;
    ///
    /// y.clone_from(&x);
    ///
    /// // The value is the same
    /// assert_eq!(x, y);
    ///
    /// // And no allocation occurred
    /// assert_eq!(yp, &*y);
    /// ```
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        (**self).clone_from(&(**source));
    }
}

impl<T: Clone, A: Allocator + Clone> Clone for TESBox<[T], A> {
    fn clone(&self) -> Self {
        let mut vec: Vec<T> = self.iter().cloned().collect();

        // -- from_vec_in --
        let len = vec.len();
        let ptr = vec.as_mut_ptr();
        // SAFETY: This was created from a Vec<T, A>
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        let slice = unsafe { NonNull::new_unchecked(slice as *mut [T]) };

        // Prevent Vec from dropping
        let _ = mem::ManuallyDrop::new(vec);

        Self(unsafe { Unique::new_unchecked(slice.as_ptr()) }, self.1.clone())
    }

    /// Copies `source`'s contents into `self` without creating a new allocation,
    /// so long as the two are of the same length.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = Box::new([5, 6, 7]);
    /// let mut y = Box::new([8, 9, 10]);
    /// let yp: *const [i32] = &*y;
    ///
    /// y.clone_from(&x);
    ///
    /// // The value is the same
    /// assert_eq!(x, y);
    ///
    /// // And no allocation occurred
    /// assert_eq!(yp, &*y);
    /// ```
    fn clone_from(&mut self, source: &Self) {
        if self.len() == source.len() {
            self.clone_from_slice(source);
        } else {
            *self = source.clone();
        }
    }
}

impl<T: ?Sized + PartialEq, A: Allocator> PartialEq for TESBox<T, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    #[inline]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}

impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for TESBox<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}

impl<T: ?Sized + Ord, A: Allocator> Ord for TESBox<T, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<T: ?Sized + Eq, A: Allocator> Eq for TESBox<T, A> {}

impl<T: ?Sized + Hash, A: Allocator> Hash for TESBox<T, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized + Hasher, A: Allocator> Hasher for TESBox<T, A> {
    fn finish(&self) -> u64 {
        (**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (**self).write(bytes);
    }
    fn write_u8(&mut self, i: u8) {
        (**self).write_u8(i);
    }
    fn write_u16(&mut self, i: u16) {
        (**self).write_u16(i);
    }
    fn write_u32(&mut self, i: u32) {
        (**self).write_u32(i);
    }
    fn write_u64(&mut self, i: u64) {
        (**self).write_u64(i);
    }
    fn write_u128(&mut self, i: u128) {
        (**self).write_u128(i);
    }
    fn write_usize(&mut self, i: usize) {
        (**self).write_usize(i);
    }
    fn write_i8(&mut self, i: i8) {
        (**self).write_i8(i);
    }
    fn write_i16(&mut self, i: i16) {
        (**self).write_i16(i);
    }
    fn write_i32(&mut self, i: i32) {
        (**self).write_i32(i);
    }
    fn write_i64(&mut self, i: i64) {
        (**self).write_i64(i);
    }
    fn write_i128(&mut self, i: i128) {
        (**self).write_i128(i);
    }
    fn write_isize(&mut self, i: isize) {
        (**self).write_isize(i);
    }
}

impl<T: fmt::Display + ?Sized, A: Allocator> fmt::Display for TESBox<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: fmt::Debug + ?Sized, A: Allocator> fmt::Debug for TESBox<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized, A: Allocator> fmt::Pointer for TESBox<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // It's not possible to extract the inner Uniq directly from the Box,
        // instead we cast it to a *const which aliases the Unique
        let ptr: *const T = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<T: ?Sized, A: Allocator> Deref for TESBox<T, A> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for TESBox<T, A> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized, A: Allocator> Borrow<T> for TESBox<T, A> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized, A: Allocator> BorrowMut<T> for TESBox<T, A> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for TESBox<T, A> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized, A: Allocator> AsMut<T> for TESBox<T, A> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

// impl<F: ?Sized + Future + Unpin, A: Allocator> Future for TESBox<F, A> {
//     type Output = F::Output;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         F::poll(Pin::new(&mut *self), cx)
//     }
// }

impl<E: Error> Error for TESBox<E> {
    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        Error::description(&**self)
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn Error> {
        Error::cause(&**self)
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(&**self)
    }
}
