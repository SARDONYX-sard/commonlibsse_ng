// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Relocation.h
// - ref(`safe_write`, `safe_fill`): https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Relocation.cpp
//
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT

use crate::rel::ResolvableAddress;
use crate::rel::id::{DataBaseError, ID, RelocationID};
use crate::rel::module::{ModuleState, ModuleStateError};
use crate::rel::offset::{Offset, VariantOffset};
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

pub fn invoke<F, Args, R>(func: F, args: Args) -> R
where
    F: FnOnce(Args) -> R,
{
    func(args)
}

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

pub(crate) fn relocate_virtual<F>(
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

/// # Errors
pub(crate) fn relocate_member<T>(
    this: *mut u8,
    se_ae_offset: isize,
    vr_offset: isize,
) -> Result<*mut T, ModuleStateError> {
    let is_vr = ModuleState::map_active(|module| module.runtime.is_vr())?;
    unsafe { Ok(this.offset(if is_vr { vr_offset } else { se_ae_offset }).cast::<T>()) }
}

pub(crate) const fn relocate_member_if<T>(
    condition: bool,
    this: *mut u8,
    a: isize,
    b: isize,
) -> *mut T {
    unsafe { this.offset(if condition { a } else { b }).cast::<T>() }
}

/// # Errors
pub(crate) fn relocate_member_if_newer<T>(
    version: crate::rel::version::Version,
    this: *mut u8,
    older: isize,
    newer: isize,
) -> Result<*mut T, ModuleStateError> {
    let current_version = ModuleState::map_active(|module| module.version.clone())?;
    unsafe { Ok(this.offset(if current_version < version { older } else { newer }).cast::<T>()) }
}
