use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::ffi::{CStr, CString};

pub struct BSString {
    data: NonNull<u8>,
    size: u16,
    capacity: u16,
    _pad0C: u32,
    marker: PhantomData<Vec<u8>>,
}

impl BSString {
    #[inline]
    pub const fn new() -> Self {
        Self { data: NonNull::dangling(), size: 0, capacity: 0, _pad0C: 0, marker: PhantomData }
    }

    #[inline]
    pub fn clear(&mut self) {
        if self.capacity > 0 {
            let mut vec = unsafe {
                Vec::from_raw_parts(self.data.as_ptr(), self.size as usize, self.capacity as usize)
            };
            vec.clear();
            #[allow(clippy::mem_forget)]
            core::mem::forget(vec);
        }
        self.data = NonNull::dangling();
        self.size = 0;
        self.capacity = 0;
    }

    pub fn set_c_str(&mut self, cstr: &CStr) {
        let bytes = cstr.to_bytes_with_nul();
        let len = bytes.len() as u16;

        if len == 0 {
            self.clear();
            return;
        }

        let mut vec = if self.size == 0 {
            Vec::with_capacity(len as usize)
        } else {
            unsafe {
                Vec::from_raw_parts(self.data.as_ptr(), self.size as usize, self.capacity as usize)
            }
        };

        if len > self.capacity {
            vec.reserve((len - self.capacity) as usize);
            self.capacity = vec.capacity() as u16;
        }

        vec.clear();
        vec.extend_from_slice(bytes);
        self.data = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };
        #[allow(clippy::mem_forget)]
        core::mem::forget(vec);

        self.size = len;
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub const fn len(&self) -> u16 {
        self.size
    }

    #[inline]
    pub const fn capacity(&self) -> u16 {
        self.capacity
    }

    #[inline]
    pub const fn as_bytes_with_null(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.size as usize) }
    }

    #[inline]
    pub const fn as_c_str(&self) -> &CStr {
        if self.size == 0 {
            return c"";
        }
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_null()) }
    }
}

impl Drop for BSString {
    fn drop(&mut self) {
        if self.capacity > 0 {
            drop(unsafe {
                Vec::from_raw_parts(self.data.as_ptr(), self.size as usize, self.capacity as usize)
            });
        }
    }
}

impl Default for BSString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BSString")
            .field("data", &self.as_c_str())
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            // .field("_pad0C", &self._pad0C)
            .finish()
    }
}

impl From<CString> for BSString {
    #[inline]
    fn from(cstring: CString) -> Self {
        let mut vec = cstring.into_bytes_with_nul();
        let ptr = vec.as_mut_ptr();
        let size = vec.len() as u16;
        let capacity = vec.capacity() as u16;

        #[allow(clippy::mem_forget)]
        core::mem::forget(vec);

        Self {
            data: unsafe { NonNull::new_unchecked(ptr) },
            size,
            capacity,
            _pad0C: 0,
            marker: PhantomData,
        }
    }
}

#[test]
fn test_bs_string() {
    let mut bs = BSString::new();

    assert_eq!(bs.len(), 0);
    assert_eq!(bs.as_c_str().to_str().unwrap(), "");

    let input = c"Hello, Rust!";
    bs.set_c_str(input);

    assert_eq!(bs.len(), 13);
    assert_eq!(bs.as_c_str().to_str().unwrap(), "Hello, Rust!");

    let input2 = c"Short";
    bs.set_c_str(input2);

    assert_eq!(bs.len(), 6);
    assert_eq!(bs.as_c_str().to_str().unwrap(), "Short");

    let input3 = c"Much longer string!";
    bs.set_c_str(input3);

    assert_eq!(bs.len(), 20);
    assert_eq!(bs.as_c_str().to_str().unwrap(), "Much longer string!");
}
