// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Relocation.h
// - ref(`safe_write`, `safe_fill`): https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Relocation.cpp
//
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
mod phantom_member;

pub use phantom_member::PhantomMember;

use crate::rel::ResolvableAddress;
use crate::rel::id::{DataBaseError, ID, RelocationID};
use crate::rel::module::{ModuleState, ModuleStateError};
use crate::rel::offset::{Offset, VariantOffset};
use crate::rel::version::Version;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem;
use core::num::NonZeroUsize;
use core::ptr::NonNull;

pub const NOP: u8 = 0x90;
pub const NOP2: [u8; 2] = [0x66, 0x90];
pub const NOP3: [u8; 3] = [0x0F, 0x1F, 0x00];
pub const NOP4: [u8; 4] = [0x0F, 0x1F, 0x40, 0x00];
pub const NOP5: [u8; 5] = [0x0F, 0x1F, 0x44, 0x00, 0x00];
pub const NOP6: [u8; 6] = [0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00];
pub const NOP7: [u8; 7] = [0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00];
pub const NOP8: [u8; 8] = [0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const NOP9: [u8; 9] = [0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const JMP8: u8 = 0xEB;
pub const JMP32: u8 = 0xE9;
pub const RET: u8 = 0xC3;
pub const INT3: u8 = 0xCC;

#[inline]
unsafe fn enable_write_permission(
    addr: *const c_void,
    len: usize,
) -> windows::core::Result<windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS> {
    unsafe {
        use windows::Win32::System::Memory::{
            PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect,
        };
        let mut old_protection = PAGE_PROTECTION_FLAGS(0);

        // VirtualProtect: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect
        VirtualProtect(addr, len, PAGE_EXECUTE_READWRITE, &mut old_protection)?;
        Ok(old_protection)
    }
}

#[inline]
unsafe fn restore_memory_protection(
    addr: *const c_void,
    len: usize,
    old_protection: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS,
) -> windows::core::Result<()> {
    unsafe {
        use windows::Win32::System::Memory::{PAGE_PROTECTION_FLAGS, VirtualProtect};
        let mut temp = PAGE_PROTECTION_FLAGS(0);

        // VirtualProtect: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect
        VirtualProtect(addr, len, old_protection, &mut temp)
    }
}

#[inline]
unsafe fn safe_write<T>(dst: *mut T, src: *const T, len: usize) -> windows::core::Result<()> {
    unsafe {
        let old_protection = enable_write_permission(dst as _, len)?;
        core::ptr::copy_nonoverlapping(src, dst, len);
        restore_memory_protection(dst.cast(), len, old_protection)
    }
}

#[inline]
pub(crate) unsafe fn safe_write_value<T>(dst: *mut T, src: &T) -> windows::core::Result<()> {
    unsafe { safe_write(dst, src, core::mem::size_of::<T>()) }
}

#[inline]
unsafe fn safe_fill(dst: *mut c_void, value: u8, len: usize) -> windows::core::Result<()> {
    unsafe {
        let old_protection = enable_write_permission(dst, len)?;
        core::ptr::write_bytes(dst, value, len);
        restore_memory_protection(dst, len, old_protection)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relocation {
    _impl: NonNull<c_void>,
    // owned
    _cast_target: PhantomData<c_void>,
}

impl Relocation {
    #[inline]
    pub const fn new(address: NonNull<c_void>) -> Self {
        Self { _impl: address, _cast_target: PhantomData }
    }

    /// Creates an instance from two resolvable addresses.
    ///
    /// # Errors
    /// Returns an error if either of the addresses cannot be resolved.
    #[inline]
    pub fn from_id_offset<A1, A2>(id: A1, offset: A2) -> Result<Self, DataBaseError>
    where
        A1: ResolvableAddress,
        A2: ResolvableAddress,
    {
        Ok(Self {
            _impl: unsafe { id.address()?.byte_add(offset.offset()?.get()) },
            _cast_target: PhantomData,
        })
    }

    /// Cast to any type.
    ///
    /// Equivalent to C++'s `REL::Relocation::get`.
    #[inline]
    pub const fn cast<U>(&self) -> NonNull<U> {
        self._impl.cast()
    }

    #[inline]
    pub fn write<U>(&self, data: &U) {
        let _ = unsafe { safe_write_value(self._impl.cast::<U>().as_mut(), data) };
    }

    #[inline]
    pub fn write_bytes(&self, data: &[u8]) {
        let _ = unsafe { safe_write(self._impl.cast::<u8>().as_mut(), data.as_ptr(), data.len()) };
    }

    #[inline]
    pub fn swap_as_vfn<T>(&mut self, idx: usize, new_fn: *const ()) -> NonNull<c_void> {
        const PTR_SIZE: usize = mem::size_of::<usize>();

        let mut old_fn = unsafe { self._impl.byte_add(PTR_SIZE * idx) };
        let _ = unsafe { safe_write(old_fn.as_mut(), new_fn.cast(), PTR_SIZE) };
        old_fn
    }

    #[inline]
    pub fn write_fill(&mut self, value: u8, count: usize) {
        unsafe {
            let _ = safe_fill(self._impl.as_mut(), value, count);
        }
    }
}

impl ResolvableAddress for Relocation {
    /// Get the address.(No error returned)
    #[inline]
    fn address(&self) -> Result<NonNull<c_void>, DataBaseError> {
        Ok(self._impl)
    }

    #[inline]
    fn offset(&self) -> Result<NonZeroUsize, DataBaseError> {
        let offset = unsafe { self._impl.byte_offset_from(Self::base()?) as usize };
        NonZeroUsize::new(offset).ok_or(DataBaseError::SpecifiedZeroOffset)
    }
}

/// # Errors
#[inline]
pub fn relocate<T>(se_and_vr: T, ae: T) -> Result<T, ModuleStateError> {
    let is_ae = ModuleState::map_or_init(|module| module.runtime.is_ae())?;
    Ok(if is_ae { ae } else { se_and_vr })
}

impl TryFrom<Offset> for Relocation {
    type Error = DataBaseError;

    #[inline]
    fn try_from(offset: Offset) -> Result<Self, Self::Error> {
        Ok(Self { _impl: offset.address()?, _cast_target: PhantomData })
    }
}

impl TryFrom<VariantOffset> for Relocation {
    type Error = DataBaseError;

    #[inline]
    fn try_from(offset: VariantOffset) -> Result<Self, Self::Error> {
        Ok(Self { _impl: offset.address()?, _cast_target: PhantomData })
    }
}

impl TryFrom<ID> for Relocation {
    type Error = DataBaseError;

    #[inline]
    fn try_from(id: ID) -> Result<Self, Self::Error> {
        Ok(Self { _impl: id.address()?, _cast_target: PhantomData })
    }
}

impl TryFrom<RelocationID> for Relocation {
    type Error = DataBaseError;

    #[inline]
    fn try_from(id: RelocationID) -> Result<Self, Self::Error> {
        Ok(Self { _impl: id.address()?, _cast_target: PhantomData })
    }
}

/// # Safety
/// # Errors
#[inline]
pub unsafe fn relocate_virtual<F>(
    se_ae_vtable_offset: isize,
    vr_vtable_offset: isize,
    se_ae_vtable_index: isize,
    vr_vtable_index: isize,
    this: NonNull<u8>,
) -> Result<F, ModuleStateError>
where
    F: Copy,
{
    let is_vr = ModuleState::map_active(|module| module.runtime.is_vr())?;

    unsafe {
        let vtable_ptr = *(this.as_ptr() as *const *const F).offset(if is_vr {
            vr_vtable_offset
        } else {
            se_ae_vtable_offset
        });
        let func_ptr = *vtable_ptr.offset(if is_vr { vr_vtable_index } else { se_ae_vtable_index });

        Ok(func_ptr)
    }
}

/// Relocates a member based on the runtime state, returning a pointer to the new location.
///
/// # Safety
/// This function requires that the caller ensure the provided pointer `this` is valid, meaning it should point to a valid memory location.
/// The `se_ae_offset` and `vr_offset` must be safe offsets for the given pointer type.
///
/// # Errors
/// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
/// Specifically, it calls `ModuleState::map_active`, which could result in an error.
pub fn relocate_member<THIS, T>(
    this: &THIS,
    se_ae_offset: isize,
    vr_offset: isize,
) -> Result<&T, RelocationError> {
    let member_ptr = {
        let is_vr = ModuleState::map_active(|module| module.runtime.is_vr())?;
        let this = this as *const THIS;
        let offset = if is_vr { vr_offset } else { se_ae_offset };
        this.wrapping_offset(offset).cast::<T>()
    };

    Ok(unsafe { raw_pointer_as_ref(member_ptr) }?)
}

/// Relocates a member based on the runtime state, returning a pointer to the new location.
///
/// # Safety
/// This function requires that the caller ensure the provided pointer `this` is valid, meaning it should point to a valid memory location.
/// The `se_ae_offset` and `vr_offset` must be safe offsets for the given pointer type.
///
/// # Errors
/// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
/// Specifically, it calls `ModuleState::map_active`, which could result in an error.
pub fn relocate_member_mut<THIS, T>(
    this: &mut THIS,
    se_ae_offset: isize,
    vr_offset: isize,
) -> Result<&mut T, RelocationError> {
    let member_ptr = {
        let is_vr = ModuleState::map_active(|module| module.runtime.is_vr())?;
        let this = this as *mut THIS;
        let offset = if is_vr { vr_offset } else { se_ae_offset };
        this.wrapping_offset(offset).cast::<T>()
    };

    Ok(unsafe { raw_pointer_as_mut(member_ptr) }?)
}

/// Relocates a member based on a condition, using either offset `a` or `b` depending on the condition.
///
/// # Safety
/// It is safe as long as the following are observed.
/// - ptr of added offset never exceeds `isize::MAX`
/// - All memory from this to offset is valid.
pub const unsafe fn relocate_member_if<T>(
    condition: bool,
    this: *mut u8,
    a: isize,
    b: isize,
) -> *mut T {
    unsafe { this.offset(if condition { a } else { b }).cast::<T>() }
}

/// Relocates a member based on the version comparison, using either `older` or `newer` offset depending on the current version.
///
/// # Safety
/// It is safe as long as the following are observed.
/// - ptr of offset added is valid
/// - ptr of added offset never exceeds `isize::MAX`
/// - All memory from this to offset is valid.
///
/// # Errors
/// - This function may return an error if the module's state cannot be accessed, or if the `map_active` call fails when fetching the current version.
/// - If the pointer is null
/// - If the pointer is unaligned
#[inline]
pub unsafe fn relocate_member_if_newer<THIS, T>(
    version: Version,
    this: &THIS,
    older: isize,
    newer: isize,
) -> Result<&T, RelocationError> {
    let is_old = ModuleState::map_active(|module| module.version < version)?;
    let this = this as *const THIS;
    let offset = if is_old { older } else { newer };
    let member_ptr = unsafe { this.offset(offset).cast::<T>() };
    Ok(unsafe { raw_pointer_as_ref(member_ptr) }?)
}

/// Relocates a member based on the version comparison, using either `older` or `newer` offset depending on the current version.
///
/// # Safety
/// It is safe as long as the following are observed.
/// - ptr of offset added is valid
/// - ptr of added offset never exceeds `isize::MAX`
/// - All memory from this to offset is valid.
///
/// # Errors
/// This function may return an error if the module's state cannot be accessed, or if the `map_active` call fails when fetching the current version.
pub unsafe fn relocate_member_if_newer_mut<THIS, T>(
    version: Version,
    this: &mut THIS,
    older: isize,
    newer: isize,
) -> Result<&mut T, RelocationError> {
    let is_old = ModuleState::map_active(|module| module.version < version)?;
    let this = this as *mut THIS;
    let offset = if is_old { older } else { newer };
    let member_ptr = unsafe { this.offset(offset).cast::<T>() };
    Ok(unsafe { raw_pointer_as_mut(member_ptr) }?)
}

/// Converts a raw pointer to an immutable reference.
///
/// # Safety
/// - The caller must guarantee that the pointer is valid.
/// - The lifetime of the reference must not outlive the pointer's validity.
///
/// # Errors
/// - If the pointer is null
/// - If the pointer is unaligned
#[inline]
pub unsafe fn raw_pointer_as_ref<'a, T>(ptr: *const T) -> Result<&'a T, RawPointerError> {
    if ptr.is_null() {
        return Err(RawPointerError::NullPointer);
    }

    if !ptr.is_aligned() {
        return Err(RawPointerError::MisalignedPointer {
            addr: ptr.addr(),
            expected_align: core::mem::align_of::<T>(),
            misaligned_type: core::any::type_name::<T>(),
        });
    };

    Ok(unsafe { &*ptr })
}

/// Converts a raw pointer to a mutable reference.
///
/// # Safety
/// - The caller must guarantee that the pointer is valid.
/// - The lifetime of the reference must not outlive the pointer's validity.
///
/// # Errors
/// - If the pointer is null
/// - If the pointer is unaligned
#[inline]
pub unsafe fn raw_pointer_as_mut<'a, T>(ptr: *mut T) -> Result<&'a mut T, RawPointerError> {
    if ptr.is_null() {
        return Err(RawPointerError::NullPointer);
    }

    if !ptr.is_aligned() {
        return Err(RawPointerError::MisalignedPointer {
            addr: ptr.addr(),
            expected_align: core::mem::align_of::<T>(),
            misaligned_type: core::any::type_name::<T>(),
        });
    }

    Ok(unsafe { &mut *ptr })
}

/// Represents errors that may occur during member relocation.
#[derive(Debug, snafu::Snafu)]
pub enum RelocationError {
    /// Error indicating issues with raw pointer conversion.
    #[snafu(transparent)]
    PointerConversion { source: RawPointerError },

    /// Error indicating that the module state could not be accessed.
    #[snafu(transparent)]
    ModuleState { source: ModuleStateError },
}

/// Represents errors that may occur when converting raw pointers to references.
#[derive(Debug, snafu::Snafu)]
pub enum RawPointerError {
    /// Null pointer encountered during conversion.
    NullPointer,

    /// Misaligned pointer.
    #[snafu(display(
        "This pointer of `{misaligned_type}` was expected to have an alignment of {expected_align}, but the actual address was {addr:X}."
    ))]
    MisalignedPointer { addr: usize, expected_align: usize, misaligned_type: &'static str },
}
