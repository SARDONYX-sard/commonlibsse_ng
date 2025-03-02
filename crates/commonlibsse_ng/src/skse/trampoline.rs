//! - ref: vcpkg_installed\x64-windows\commonlibsse_ng\include\SKSE\Trampoline.h

use crate::rel::module::{ModuleState, SegmentName};
use snafu::ResultExt as _;
use std::{
    collections::HashMap,
    ffi::c_void,
    mem, ptr,
    sync::{OnceLock, RwLock},
};
use windows::Win32::System::Memory::{MEM_FREE, MEM_RELEASE, VirtualFree};

#[inline]
const fn round_up(value: usize, multiple: usize) -> usize {
    (value + multiple - 1) & !(multiple - 1)
}

#[inline]
const fn round_down(value: usize, multiple: usize) -> usize {
    value & !(multiple - 1)
}

#[inline]
const fn in_i32range(disp: isize) -> bool {
    disp >= (i32::MIN as isize) && (disp <= i32::MAX as isize)
}

#[derive(Debug)]
#[repr(C, packed)]
struct SrcAssembly {
    // jmp(0xe9) or call(0xe8)
    opcode: u8, // 0 - 0xE9/0xE8
    disp: i32,  // 1
}

/// A jump instruction (JMP `[rip]`) in FF 25 format.
#[derive(Debug)]
#[repr(C, packed)]
struct TrampolineAssembly {
    // jmp: 0xFF (indirect jump instruction).
    jmp: u8, // 0 - 0xFF
    // modrm: 0x25 (mode/register/memory operand specification).
    modrm: u8, // 1 - 0x25
    // disp: displacement (normally 0).
    disp: i32, // 2 - 0x00000000
    /// addr: actual jump destination address.
    addr: u64, // 6 - [rip]
}

#[derive(Debug)]
#[repr(C, packed)]
struct Assembly {
    opcode: u8, // 0 - 0xFF
    modrm: u8,  // 1 - 0x25/0x15
    disp: i32,  // 2
}

type Deleter = Option<Box<dyn Fn(*mut c_void, usize) + Send>>;

pub struct Trampoline {
    name: String,
    data: *mut u8,
    capacity: usize,
    size: usize,
    /// Maybe
    /// - key: fn_ptr, value: data
    branches_5: HashMap<*mut c_void, *mut u8>,
    /// Maybe
    /// - key: fn_ptr, value: data
    branches_6: HashMap<*mut c_void, *mut u8>,
    deleter: Deleter,
}

unsafe impl Send for Trampoline {} // FIXME: dummy
unsafe impl Sync for Trampoline {}

impl core::fmt::Debug for Trampoline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trampoline")
            .field("name", &self.name)
            .field("data", &self.data)
            .field("capacity", &self.capacity)
            .field("size", &self.size)
            .field("branches_5", &self.branches_5)
            .field("branches_6", &self.branches_6)
            .field("deleter", &{
                match self.deleter {
                    Some(_) => "Some(Box<dyn Fn(*mut c_void, usize) + Send>)",
                    None => "None",
                }
            })
            .finish()
    }
}

impl Default for Trampoline {
    #[inline]
    fn default() -> Self {
        Self::new("Default Trampoline")
    }
}

impl Trampoline {
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: ptr::null_mut(),
            capacity: 0,
            size: 0,
            branches_5: HashMap::new(),
            branches_6: HashMap::new(),
            deleter: None,
        }
    }

    /// Allocates a memory block for trampoline execution.
    ///
    /// # Errors
    /// Returns `TrampolineError::FailedToAllocate` if memory allocation fails.
    pub fn create(
        &mut self,
        size: usize,
        module: Option<*mut c_void>,
    ) -> Result<(), TrampolineError> {
        if size == 0 {
            return Err(TrampolineError::InvalidCreateZeroSize);
        }

        let module = if let Some(module) = module {
            module
        } else {
            let text = ModuleState::map_or_init(|module| module.segment(SegmentName::Textx))?;
            text.proxy_base.0.as_ptr().with_addr((text.address + text.size) as usize)
        };

        let mem = Self::do_create(size, module)?;

        let deleter: Deleter = Some(Box::new(|mem, _size| {
            if let Err(err) = unsafe { VirtualFree(mem, 0, MEM_RELEASE) } {
                #[cfg(feature = "tracing")]
                tracing::error!("VirtualFree failed: {err}");
            };
        }));
        unsafe { self.set_trampoline(mem, size, deleter) };

        Ok(())
    }

    /// # Safety
    #[inline]
    pub unsafe fn set_trampoline(&mut self, trampoline: *mut u8, size: usize, deleter: Deleter) {
        if !trampoline.is_null() {
            const INT3: u8 = 0xCC;
            unsafe { ptr::write_bytes(trampoline, INT3, size) };
        }
        self.release();

        self.data = trampoline;
        self.capacity = size;
        self.size = 0;
        self.deleter = deleter;
    }

    /// # Errors
    #[inline]
    pub fn allocate_size_of<T>(&mut self) -> Result<*mut u8, TrampolineError> {
        self.allocate(size_of::<T>())
    }

    /// # Errors
    #[inline]
    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, TrampolineError> {
        self.do_allocate(size)
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.capacity == 0
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub const fn allocated_size(&self) -> usize {
        self.size
    }

    #[inline]
    pub const fn free_size(&self) -> usize {
        self.capacity.saturating_sub(self.size)
    }

    /// # Errors
    /// # Safety
    #[inline]
    pub unsafe fn write_branch<B>(
        &mut self,
        src: *mut c_void,
        dst: *mut c_void,
    ) -> Result<*mut c_void, TrampolineError>
    where
        B: BranchKind,
    {
        unsafe { self.write_branch_with_data::<B>(src, dst, B::jmp_size()) }
    }

    /// # Errors
    /// # Safety
    #[inline]
    pub unsafe fn write_call<B>(
        &mut self,
        src: *mut c_void,
        dst: *mut c_void,
    ) -> Result<*mut c_void, TrampolineError>
    where
        B: BranchKind,
    {
        unsafe { self.write_branch_with_data::<B>(src, dst, B::call_size()) }
    }

    fn do_create(size: usize, address: *mut c_void) -> Result<*mut u8, TrampolineError> {
        use std::mem::size_of;
        use windows::Win32::System::Memory::{
            MEM_COMMIT, MEM_RESERVE, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE,
            VirtualAlloc, VirtualQuery,
        };
        use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

        const GIGABYTE: usize = 1 << 30;
        const MIN_RANGE: usize = GIGABYTE * 2;
        const MAX_ADDR: usize = usize::MAX;

        let address = address as usize; // FIXME

        let mut si = SYSTEM_INFO::default();
        unsafe { GetSystemInfo(&mut si) };
        let granularity = si.dwAllocationGranularity as usize;

        let mut min =
            if address >= MIN_RANGE { round_up(address - MIN_RANGE, granularity) } else { 0 };
        let max = if address < (MAX_ADDR - MIN_RANGE) {
            round_down(address + MIN_RANGE, granularity)
        } else {
            MAX_ADDR
        };

        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        while min < max {
            if unsafe {
                VirtualQuery(Some(min as *const _), &mut mbi, size_of::<MEMORY_BASIC_INFORMATION>())
            } == 0
            {
                return Err(TrampolineError::FailedToGetMemInfo {
                    source: windows::core::Error::from_win32(),
                });
            }

            let base_addr = mbi.BaseAddress as usize;
            min = base_addr + mbi.RegionSize;

            if mbi.State == MEM_FREE {
                let addr = round_up(base_addr, granularity);
                if addr < min && (min - addr) >= size {
                    let mem = unsafe {
                        VirtualAlloc(
                            Some(addr as *mut _),
                            size,
                            MEM_COMMIT | MEM_RESERVE,
                            PAGE_EXECUTE_READWRITE,
                        )
                    };
                    if !mem.is_null() {
                        return Ok(mem.cast::<u8>());
                    }
                    return Err(TrampolineError::FailedToAllocate {
                        source: windows::core::Error::from_win32(),
                    });
                }
            }
        }

        Err(TrampolineError::FailedToAllocate { source: windows::core::Error::from_win32() })
    }

    fn do_allocate(&mut self, size: usize) -> Result<*mut u8, TrampolineError> {
        if size > self.free_size() {
            return Err(TrampolineError::NoMemorySpace);
            // panic!("Failed to handle allocation request");
        }
        let mem = unsafe { self.data.add(self.size) };
        self.size += size;
        Ok(mem)
    }

    unsafe fn write_5branch(
        &mut self,
        src: *mut SrcAssembly,
        dst: *mut c_void,
        opcode: u8,
    ) -> Result<(), TrampolineError> {
        let mem = if let Some(mem) = self.branches_5.get(&dst).copied() {
            mem
        } else {
            let mem = self.allocate_size_of::<usize>()?;
            self.branches_5.insert(dst, mem);
            mem
        };

        let disp = (mem.addr() as isize) - ((src.addr() + mem::size_of::<SrcAssembly>()) as isize);
        if !in_i32range(disp) {
            return Err(TrampolineError::OutOfRangeDisplacement { displacement: disp });
        }

        let assembly = SrcAssembly { opcode, disp: disp as i32 };
        unsafe { crate::rel::relocation::safe_write_value(src, &assembly) }
            .context(FailedToWriteMemorySnafu)?;

        let mem = mem.cast::<TrampolineAssembly>();
        unsafe {
            mem.write_unaligned(TrampolineAssembly {
                jmp: 0xFF,
                modrm: 0x25,
                disp: 0,
                addr: dst as u64,
            });
        }

        Ok(())
    }

    unsafe fn write_6branch(
        &mut self,
        src: *mut Assembly,
        dst: *mut c_void,
        modrm: u8,
    ) -> Result<(), TrampolineError> {
        use crate::rel::relocation::safe_write_value;

        let mem = if let Some(mem) = self.branches_6.get(&dst).copied() {
            mem
        } else {
            let mem = self.allocate_size_of::<usize>()?;
            self.branches_6.insert(dst, mem);
            mem
        };

        let disp = (mem.addr() as isize) - ((src.addr() + mem::size_of::<Assembly>()) as isize);
        if !in_i32range(disp) {
            return Err(TrampolineError::OutOfRangeDisplacement { displacement: disp });
        }

        let assembly = Assembly { opcode: 0xFF, modrm, disp: disp as i32 };
        unsafe { safe_write_value(src, &assembly) }.context(FailedToWriteMemorySnafu)?;

        let mem = mem.cast::<usize>();
        unsafe {
            mem.write_unaligned(dst.addr());
        }

        Ok(())
    }

    /// Return next op function pointer.
    ///
    /// # Errors
    /// # Safety
    #[inline]
    pub unsafe fn write_branch_with_data<N>(
        &mut self,
        src: *mut c_void,
        dst: *mut c_void,
        data: u8,
    ) -> Result<*mut c_void, TrampolineError>
    where
        N: BranchKind,
    {
        let kind = N::kind();
        let fn_ptr = {
            let kind = kind as usize;

            let src = src.cast::<u8>();

            dbg!(src as *const _);
            let assembly = unsafe { ptr::read_unaligned(src.add(kind - 4) as *const Assembly) };
            dbg!(&assembly);

            let disp = src.map_addr(|addr| addr + kind - 4).cast::<i32>();
            let next_op = src.wrapping_add(kind);
            unsafe { next_op.add(ptr::read_unaligned(disp) as usize).cast() } // `func = nextOp + *disp`
        };

        match kind {
            BranchKindValue::Branch5 => unsafe { self.write_5branch(src.cast(), dst, data) }?,
            BranchKindValue::Branch6 => unsafe { self.write_6branch(src.cast(), dst, data) }?,
        }

        Ok(fn_ptr)
    }

    fn release(&mut self) {
        if !self.data.is_null() {
            if let Some(deleter) = &self.deleter {
                deleter(self.data.cast::<c_void>(), self.capacity);
            }
        }

        self.branches_5.clear();
        self.branches_6.clear();
        self.data = ptr::null_mut();
        self.capacity = 0;
        self.size = 0;
    }
}

impl Drop for Trampoline {
    #[inline]
    fn drop(&mut self) {
        self.release();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BranchKindValue {
    Branch5 = 5,
    Branch6 = 6,
}

#[allow(private_bounds)]
pub trait BranchKind: private::Sealed {
    fn kind() -> BranchKindValue;
    fn call_size() -> u8;
    fn jmp_size() -> u8;
}

mod private {
    pub(crate) trait Sealed {}

    impl Sealed for super::Branch5 {}
    impl Sealed for super::Branch6 {}
}

pub enum Branch5 {}
pub enum Branch6 {}

impl BranchKind for Branch5 {
    fn kind() -> BranchKindValue {
        BranchKindValue::Branch5
    }

    /// CALL rel32
    #[inline]
    fn call_size() -> u8 {
        0xE8
    }

    /// JMP rel32
    #[inline]
    fn jmp_size() -> u8 {
        0xE9
    }
}

impl BranchKind for Branch6 {
    fn kind() -> BranchKindValue {
        BranchKindValue::Branch6
    }

    /// CALL r/m64
    #[inline]
    fn call_size() -> u8 {
        0x15
    }

    /// JMP r/m64
    #[inline]
    fn jmp_size() -> u8 {
        0x25
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum TrampolineError {
    /// Invalid branch size: {size}
    InvalidBranchSize { size: usize },

    /// Expected `displacement` is i32 range: i32::MIN <= `displacement` <= i32::MAX, but got {displacement}
    #[snafu(display(
        "Expected `displacement` is i32 range: i32::MIN({})<= `displacement` <= i32::MAX({}), but got {displacement}",
        i32::MIN,
        i32::MAX
    ))]
    OutOfRangeDisplacement { displacement: isize },

    /// Cannot create a trampoline with a size of zero
    InvalidCreateZeroSize,

    /// No memory space available for trampoline
    NoMemorySpace,

    /// Failed to acquire memory information immediately before allocating.
    FailedToGetMemInfo { source: windows::core::Error },

    /// Failed to write to the memory: {source}
    FailedToWriteMemory { source: windows::core::Error },

    /// Failed to allocate memory: {source}
    FailedToAllocate { source: windows::core::Error },

    /// Failed to free memory: {source}
    FailedToFree { source: windows::core::Error },

    /// Inherited module state(manager) get error.
    #[snafu(transparent)]
    ModuleStateError { source: crate::rel::module::ModuleStateError },
}

pub fn get_trampoline() -> &'static RwLock<Trampoline> {
    static TRAMPOLINE: OnceLock<RwLock<Trampoline>> = OnceLock::new();
    TRAMPOLINE.get_or_init(|| RwLock::new(Trampoline::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel::{ResolvableAddress as _, id::VariantID, relocation::Relocation};
    use core::mem;
    use std::sync::atomic::AtomicBool;

    #[ignore = "Still can't pass. `retour-rs`, `dll-syringe` is helpful"]
    #[test]
    fn test_trampoline_write_branch() {
        static SUCCESS: AtomicBool = AtomicBool::new(false);

        extern "C" fn test_hook() {
            SUCCESS.store(true, std::sync::atomic::Ordering::Release);
        }

        let mut trampoline = Trampoline::new("trampoline");
        let fn_id = VariantID::new(102625, 110073, 0x0);
        let call = {
            let addr = fn_id.address().map(|addr| unsafe { addr.byte_add(0x4) }).unwrap();
            Relocation::new(addr)
        };

        #[allow(clippy::fn_to_numeric_cast_any)]
        let target_ptr = test_hook as *mut c_void;
        unsafe { trampoline.write_call::<Branch6>(call.address().unwrap(), target_ptr).unwrap() };
    }

    #[ignore = "Still can't pass. `retour-rs`, `dll-syringe` is helpful"]
    #[test]
    #[allow(clippy::fn_to_numeric_cast_any)]
    #[allow(clippy::missing_const_for_fn)]
    fn test_trampoline_hook() {
        #[inline(never)]
        unsafe extern "C" fn ret5() -> i32 {
            5
        }

        #[inline(never)]
        unsafe extern "C" fn ret10() -> i32 {
            10
        }

        let mut trampoline = Trampoline::new("test_trampoline");
        trampoline.create(32, None).unwrap_or_else(|err| panic!("{err}"));

        let original_ret5: unsafe extern "C" fn() -> i32 = ret5;

        // Hook ret5 to point to ret10
        let _next_fn_ptr = unsafe {
            trampoline.write_branch::<Branch5>(ret5 as *mut c_void, ret10 as *mut c_void)
        }
        .unwrap_or_else(|err| panic!("{err}"));

        let new_ret5: unsafe extern "C" fn() -> i32 =
            unsafe { mem::transmute(ret5 as *mut c_void) };

        assert_eq!(unsafe { new_ret5() }, 10); // Verify that calling ret5 now returns 10
        drop(trampoline); // Unhook and ensure original function works

        assert_eq!(unsafe { original_ret5() }, 5);
    }

    #[test]
    #[allow(clippy::fn_to_numeric_cast_any)]
    #[allow(clippy::missing_const_for_fn)]
    fn raw() -> Result<(), retour::Error> {
        use retour::GenericDetour;

        fn add5(val: i32) -> i32 {
            val + 5
        }

        fn add10(val: i32) -> i32 {
            val + 10
        }

        let hook = unsafe { GenericDetour::<fn(i32) -> i32>::new(add5, add10)? };

        assert_eq!(add5(5), 10);
        assert_eq!(hook.call(5), 10);

        unsafe { hook.enable()? };

        assert_eq!(add5(5), 15);
        assert_eq!(hook.call(5), 10);

        unsafe { hook.disable()? };

        assert_eq!(add5(5), 10);
        Ok(())
    }
}
