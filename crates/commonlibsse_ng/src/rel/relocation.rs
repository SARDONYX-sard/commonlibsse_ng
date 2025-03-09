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
use core::{mem, ptr};

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
    _impl: *mut c_void,
    // owned
    _cast_target: PhantomData<c_void>,
}

impl Relocation {
    #[inline]
    pub const fn new(address: *mut c_void) -> Self {
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
            _impl: unsafe { id.address()?.byte_add(offset.offset()?) },
            _cast_target: PhantomData,
        })
    }

    /// Cast to any type.
    ///
    /// Equivalent to C++'s `REL::Relocation::get`.
    ///
    /// # Note
    /// Null ptr to `Option::None`
    #[inline]
    pub const fn cast<U>(&self) -> Option<*mut U> {
        if self._impl.is_null() { None } else { Some(self._impl.cast()) }
    }

    #[inline]
    pub fn write<U>(&self, data: &U) {
        let _ = unsafe { safe_write_value(self._impl.cast::<U>(), data) };
    }

    #[inline]
    pub fn write_bytes(&self, data: &[u8]) {
        let _ = unsafe { safe_write(self._impl.cast::<u8>(), data.as_ptr(), data.len()) };
    }

    #[inline]
    pub fn write_vfunc(&self, idx: usize, new_func: usize) -> usize {
        const PTR_SIZE: usize = mem::size_of::<usize>();

        let addr = unsafe { self._impl.byte_add(PTR_SIZE * idx) };
        let old_func = unsafe { ptr::read(addr as *const usize) };
        let _ = unsafe { safe_write_value(addr.cast::<usize>(), &new_func) };
        old_func
    }

    #[inline]
    pub fn write_fill(&self, value: u8, count: usize) {
        unsafe {
            let _ = safe_fill(self._impl, value, count);
        }
    }
}

impl ResolvableAddress for Relocation {
    /// Get the address.(No error returned)
    #[inline]
    fn address(&self) -> Result<*mut c_void, DataBaseError> {
        Ok(self._impl)
    }

    #[inline]
    fn offset(&self) -> Result<usize, DataBaseError> {
        Ok(unsafe { self._impl.byte_offset_from(Self::base()?.as_ptr()) as usize })
    }
}

/// # Errors
#[inline]
pub fn relocate<T>(se_and_vr: T, ae: T) -> Result<T, ModuleStateError> {
    let is_ae = ModuleState::map_or_init(|module| module.runtime.is_ae())?;
    Ok(if is_ae { ae } else { se_and_vr })
}

impl<T> From<*mut T> for Relocation {
    #[inline]
    fn from(address: *mut T) -> Self {
        Self::new(address.cast())
    }
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
