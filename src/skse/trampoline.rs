//! - ref: vcpkg_installed\x64-windows\commonlibsse_ng\include\SKSE\Trampoline.h

use crate::rel::module::{ModuleState, SegmentName};
use std::{collections::HashMap, ffi::c_void, mem, ptr};
use windows::Win32::System::Memory::{VirtualFree, MEM_FREE, MEM_RELEASE};

#[inline]
const fn round_up(value: usize, multiple: usize) -> usize {
    (value + multiple - 1) & !(multiple - 1)
}

#[inline]
const fn round_down(value: usize, multiple: usize) -> usize {
    value & !(multiple - 1)
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

type Deleter = Option<Box<dyn Fn(*mut c_void, usize)>>;

pub struct Trampoline {
    name: String,
    data: *mut u8,
    capacity: usize,
    size: usize,
    branches_5: HashMap<usize, *mut u8>,
    branches_6: HashMap<usize, *mut u8>,
    deleter: Deleter,
}

impl core::fmt::Debug for Trampoline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trampoline")
            .field("name", &self.name)
            .field("data", &self.data)
            .field("capacity", &self.capacity)
            .field("size", &self.size)
            .field("branches_5", &self.branches_5)
            .field("branches_6", &self.branches_6)
            // .field("deleter", &self.deleter)
            .finish()
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

    const INT3: u8 = 0xCC;

    /// # Panics
    pub fn create(&mut self, size: usize, mut module: *mut u8) {
        if size == 0 {
            panic!("Cannot create a trampoline with a size of zero");
        }

        if module.is_null() {
            let text =
                ModuleState::map_or_init(|module| module.segment(SegmentName::Textx)).unwrap();
            module = (text.address + text.size) as _;
        }

        let mem = Self::do_create(size, module as usize);

        match mem {
            Some(mem) => {
                let deleter: Deleter = Some(Box::new(|mem, _size| {
                    let _ = unsafe { VirtualFree(mem, 0, MEM_RELEASE) };
                }));
                unsafe { self.set_trampoline(mem, size, deleter) };
            }
            None => {
                panic!("Failed to create trampoline.");
            }
        }
    }

    /// # Safety
    #[inline]
    pub unsafe fn set_trampoline(&mut self, trampoline: *mut u8, size: usize, deleter: Deleter) {
        if !trampoline.is_null() {
            unsafe { ptr::write_bytes(trampoline, Self::INT3, size) };
        }
        self.release();

        self.data = trampoline;
        self.capacity = size;
        self.size = 0;
        self.deleter = deleter;
    }

    #[inline]
    pub fn allocate_size_of<T>(&mut self) -> *mut u8 {
        self.allocate(size_of::<T>())
    }

    /// C++: `do_allocate`
    #[inline]
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        if size > self.free_size() {
            return ptr::null_mut();
        }
        let ptr = unsafe { self.data.byte_add(self.size) };
        self.size += size;
        ptr
    }

    #[inline]
    pub const fn empty(&self) -> bool {
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

    /// # Safety
    /// # Panics
    #[inline]
    pub unsafe fn write_branch<const N: usize>(&mut self, a_src: usize, a_dst: usize) -> usize {
        let data: u8 = match N {
            5 => 0xE9, // JMP rel32
            6 => 0x25, // JMP r/m64
            _ => panic!("invalid branch size"),
        };

        self.write_branch_with_data::<N>(a_src, a_dst, data)
    }

    /// # Safety
    /// # Panics
    #[inline]
    pub unsafe fn write_call<const N: usize>(&mut self, a_src: usize, a_dst: usize) -> usize {
        let data: u8 = match N {
            5 => 0xE8, // CALL rel32
            6 => 0x15, // CALL r/m64
            _ => panic!("invalid call size"),
        };

        self.write_branch_with_data::<N>(a_src, a_dst, data)
    }

    fn do_create(size: usize, address: usize) -> Option<*mut u8> {
        use std::mem::size_of;
        use windows::Win32::System::Memory::{
            VirtualAlloc, VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        };
        use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

        const GIGABYTE: usize = 1 << 30;
        const MIN_RANGE: usize = GIGABYTE * 2;
        const MAX_ADDR: usize = usize::MAX;

        let mut si = SYSTEM_INFO::default();
        unsafe { GetSystemInfo(&mut si) };
        let granularity = si.dwAllocationGranularity as usize;

        let mut min = if address >= MIN_RANGE {
            round_up(address - MIN_RANGE, granularity)
        } else {
            0
        };
        let max = if address < (MAX_ADDR - MIN_RANGE) {
            round_down(address + MIN_RANGE, granularity)
        } else {
            MAX_ADDR
        };

        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        while min < max {
            if unsafe {
                VirtualQuery(
                    Some(min as *const _),
                    &mut mbi,
                    size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            } == 0
            {
                eprintln!(
                    "VirtualQuery failed with code: 0x{:08X}",
                    unsafe { windows::Win32::Foundation::GetLastError() }.0
                );
                return None;
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
                        return Some(mem.cast::<u8>());
                    }
                    eprintln!(
                        "VirtualAlloc failed with code: 0x{:08X}",
                        unsafe { windows::Win32::Foundation::GetLastError() }.0
                    );
                }
            }
        }
        None
    }

    fn do_allocate(&mut self, size: usize) -> Option<*mut u8> {
        if size > self.free_size() {
            panic!("Failed to handle allocation request");
        }
        let mem = unsafe { self.data.add(self.size) };
        self.size += size;
        Some(mem)
    }

    unsafe fn write_5branch(&mut self, a_src: usize, a_dst: usize, a_opcode: u8) {
        let mem = self.branches_5.get(&a_dst).copied().map_or_else(
            || {
                let mem = self.allocate_size_of::<usize>();
                self.branches_5.insert(a_dst, mem);
                mem
            },
            |v| v,
        );

        let disp = (mem as isize) - (a_src + mem::size_of::<SrcAssembly>()) as isize;
        if !Self::in_range(disp) {
            panic!("displacement is out of range");
        }

        let assembly = SrcAssembly {
            opcode: a_opcode,
            disp: disp as i32,
        };
        let _ = crate::rel::relocation::safe_write_value(a_src as *mut SrcAssembly, &assembly);

        let mem = *mem as *mut TrampolineAssembly;
        (*mem).jmp = 0xFF;
        (*mem).modrm = 0x25;
        (*mem).disp = 0;
        (*mem).addr = a_dst as u64;
    }

    unsafe fn write_6branch(&mut self, a_src: usize, a_dst: usize, a_modrm: u8) {
        let mem = self.branches_6.get(&a_dst).copied().map_or_else(
            || {
                let mem = self.allocate_size_of::<usize>();
                self.branches_6.insert(a_dst, mem);
                mem
            },
            |v| v,
        );

        let disp = (mem as isize) - ((a_src + mem::size_of::<Assembly>()) as isize);
        if !Self::in_range(disp) {
            panic!("displacement is out of range");
        }

        let assembly = Assembly {
            opcode: 0xFF,
            modrm: a_modrm,
            disp: disp as i32,
        };
        let _ = crate::rel::relocation::safe_write_value(a_src as *mut Assembly, &assembly);

        let mem = *mem as *mut usize;
        *mem = a_dst;
    }

    /// # Safety
    #[inline]
    pub unsafe fn write_branch_with_data<const N: usize>(
        &mut self,
        a_src: usize,
        a_dst: usize,
        data: u8,
    ) -> usize {
        const { assert!(N == 5 || N == 6) };

        if N == 5 {
            self.write_5branch(a_src, a_dst, data);
        } else if N == 6 {
            self.write_6branch(a_src, a_dst, data);
        } else {
            panic!("Invalid branch size");
        }

        let disp = (a_src + N - 4) as *const usize;
        let next_op = a_src + N;
        next_op + unsafe { *disp } // return function
    }

    #[inline]
    const fn in_range(disp: isize) -> bool {
        disp >= (i32::MIN as isize) && (disp <= i32::MAX as isize)
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
