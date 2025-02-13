// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Relocation.h
// - ref(`safe_write`, `safe_fill`): https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Relocation.cpp
//
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT

use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::rel::id::{DataBaseError, RelocationID, ID};
use crate::rel::module::{ModuleState, ModuleStateError};
use crate::rel::offset::{Offset, VariantOffset};
use crate::rel::ResolvableAddress;

pub trait MeetsLengthReq {}

macro_rules! impl_meets_length_req {
            ($($t:ty),*) => {
                $(impl MeetsLengthReq for $t {})*
            };
        }

impl_meets_length_req!(u8, u16, u32, u64);

pub trait MeetsFunctionReq {}
pub trait MeetsMemberReq {}
pub trait IsX64Pod {}

pub struct MemberFunctionPodType<F>(PhantomData<F>);
pub struct MemberFunctionNonPodType<F>(PhantomData<F>);

pub fn invoke_member_function_non_pod<F, First, Rest>(func: F, first: First, rest: Rest) -> First
where
    F: Fn(First, *mut First, Rest),
{
    let mut result = MaybeUninit::uninit();
    func(first, result.as_mut_ptr(), rest);
    unsafe { result.assume_init() }
}

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
    addr: *const core::ffi::c_void,
    len: usize,
) -> windows::core::Result<windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS> {
    use windows::Win32::System::Memory::{
        VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
    };
    let mut old_protection = PAGE_PROTECTION_FLAGS(0);

    // VirtualProtect: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect
    VirtualProtect(addr, len, PAGE_EXECUTE_READWRITE, &mut old_protection)?;
    Ok(old_protection)
}

#[inline]
unsafe fn restore_memory_protection(
    addr: *const core::ffi::c_void,
    len: usize,
    old_protection: windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS,
) -> windows::core::Result<()> {
    use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS};
    let mut temp = PAGE_PROTECTION_FLAGS(0);

    // VirtualProtect: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect
    VirtualProtect(addr, len, old_protection, &mut temp)
}

#[inline]
unsafe fn safe_write<T>(dst: *mut T, src: *const T, len: usize) -> windows::core::Result<()> {
    let old_protection = enable_write_permission(dst as _, len)?;
    core::ptr::copy_nonoverlapping(src, dst, len);
    restore_memory_protection(dst as _, len, old_protection)
}

#[inline]
unsafe fn safe_write_value<T>(dst: *mut T, src: &T) -> windows::core::Result<()> {
    safe_write(dst, src, core::mem::size_of::<T>())
}

#[allow(unused)]
#[inline]
unsafe fn safe_fill(
    dst: *const core::ffi::c_void,
    value: u8,
    len: usize,
) -> windows::core::Result<()> {
    let old_protection = enable_write_permission(dst, len)?;
    core::ptr::write_bytes(dst as *mut u8, value, len);
    restore_memory_protection(dst, len, old_protection)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Relocation<T = usize> {
    _impl: usize,
    _marker: PhantomData<T>,
}

impl<T> Relocation<T> {
    #[inline]
    pub const fn new(address: usize) -> Self {
        Self {
            _impl: address,
            _marker: PhantomData,
        }
    }

    /// Creates an instance from two resolvable addresses.
    ///
    /// # Errors
    /// Returns an error if either of the addresses cannot be resolved.
    #[inline]
    pub fn from_addresses<A1, A2>(id: A1, offset: A2) -> Result<Self, DataBaseError>
    where
        A1: ResolvableAddress,
        A2: ResolvableAddress,
    {
        Ok(Self {
            _impl: id.address()? + offset.offset()?,
            _marker: PhantomData,
        })
    }

    #[inline]
    pub const fn address(&self) -> usize {
        self._impl
    }

    /// # Errors
    #[inline]
    pub fn offset(&self) -> Result<usize, ModuleStateError> {
        Ok(self._impl - Self::base()?)
    }

    /// # Panics
    #[inline]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        assert!(self._impl != 0);
        unsafe { ptr::read(self._impl as *const T) }
    }

    #[inline]
    pub fn write<U>(&self, data: &U)
    where
        U: Into<usize>,
    {
        let _ = unsafe { safe_write_value(self._impl as *mut U, data) };
    }

    #[inline]
    pub fn write_bytes(&self, data: &[u8])
    where
        T: Into<usize>,
    {
        let _ = unsafe { safe_write(self._impl as *mut u8, data.as_ptr(), data.len()) };
    }

    #[inline]
    pub fn write_vfunc(&self, idx: usize, new_func: usize) -> usize
    where
        T: Into<usize>,
    {
        let addr = self._impl + (mem::size_of::<usize>() * idx);
        let old_func = unsafe { ptr::read(addr as *const usize) };
        let _ = unsafe { safe_write_value(addr as *mut usize, &new_func) };
        old_func
    }

    #[inline]
    pub fn write_fill(&self, value: u8, count: usize)
    where
        T: Into<usize>,
    {
        unsafe {
            ptr::write_bytes(self._impl as *mut u8, value, count);
        }
    }

    /// # Errors
    #[inline]
    pub fn base() -> Result<usize, ModuleStateError> {
        ModuleState::map_or_init(|module| module.base.as_raw())
    }
}

impl<T> Deref for Relocation<T>
where
    T: Copy,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self._impl as *const T) }
    }
}

impl<T> DerefMut for Relocation<T>
where
    T: Copy,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self._impl as *mut T) }
    }
}

/// # Errors
pub fn relocate<T>(se_and_vr: T, ae: T) -> Result<T, ModuleStateError> {
    let runtime = ModuleState::map_or_init(|module| module.runtime)?;
    Ok(if runtime.is_ae() { ae } else { se_and_vr })
}

impl<T> From<usize> for Relocation<T> {
    fn from(address: usize) -> Self {
        Self::new(address)
    }
}

impl<T> TryFrom<Offset> for Relocation<T> {
    type Error = DataBaseError;

    #[inline]
    fn try_from(offset: Offset) -> Result<Self, Self::Error> {
        Ok(Self {
            _impl: offset.address()?,
            _marker: PhantomData,
        })
    }
}

impl<T> TryFrom<VariantOffset> for Relocation<T> {
    type Error = DataBaseError;

    #[inline]
    fn try_from(offset: VariantOffset) -> Result<Self, Self::Error> {
        Ok(Self {
            _impl: offset.address()?,
            _marker: PhantomData,
        })
    }
}

impl<T> TryFrom<ID> for Relocation<T> {
    type Error = DataBaseError;

    #[inline]
    fn try_from(id: ID) -> Result<Self, Self::Error> {
        Ok(Self {
            _impl: id.address()?,
            _marker: PhantomData,
        })
    }
}

impl<T> TryFrom<RelocationID> for Relocation<T> {
    type Error = DataBaseError;

    #[inline]
    fn try_from(id: RelocationID) -> Result<Self, Self::Error> {
        Ok(Self {
            _impl: id.address()?,
            _marker: PhantomData,
        })
    }
}
