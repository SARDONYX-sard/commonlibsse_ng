use core::ffi::{CStr, c_char, c_void};
use core::fmt;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ValueType {
    #[default]
    Undefined = 0x00,
    Null = 0x01,
    Boolean = 0x02,
    Number = 0x03,
    String = 0x04,
    StringW = 0x05,
    Object = 0x06,
    Array = 0x07,
    DisplayObject = 0x08,

    ManagedBit = 1 << 6,
    ConvertBit = 1 << 7,
    ValueMask = 0x0F,

    /// `ConvertBit` | `ValueMask`
    TypeMask = 1 << 7 | 0x0F,
    // `ConvertBit` | `Boolean`,
    ConvertBoolean = 1 << 7 | 0x02,
    // `ConvertBit` | `Number`,
    ConvertNumber = 1 << 7 | 0x03,
    // `ConvertBit` | `String`,
    ConvertString = 1 << 7 | 0x04,
    // `ConvertBit` | `StringW`,
    ConvertStringW = 1 << 7 | 0x05,
}

#[repr(C)]
#[derive(Debug)]
pub struct ObjectInterface {
    pub movieRoot: *mut (),
}

impl ObjectInterface {
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 80222, ae_id = 82245)]
    pub unsafe fn get_member(
        &self,
        data: *mut c_void,
        name: *const c_char,
        value: &mut GFxValue,
        is_d_obj: bool,
    ) {
    }
}

union ValueUnion {
    number: f64,
    boolean: bool,
    string: *const c_char,
    managedString: *mut *const c_char,
    wideString: *const u16,             // wchar_t
    managedWideString: *mut *const u16, // wchar_t
    obj: *mut c_void,
}

impl Default for ValueUnion {
    fn default() -> Self {
        Self { obj: core::ptr::null_mut() }
    }
}

pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(*const c_char),
    ManagedString(*mut *const c_char),
    WideString(*const u16),             // wchar_t
    ManagedWideString(*mut *const u16), // wchar_t
    Object(*mut c_void),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => write!(f, "Value(Undefined)"),
            Self::Null => write!(f, "Value(Null)"),
            Self::Boolean(b) => write!(f, "Value(Boolean({}))", b),
            Self::Number(n) => write!(f, "Value(Number({}))", n),
            Self::String(s) => write!(f, "Value(String({:?}))", unsafe { CStr::from_ptr(*s) }),
            Self::ManagedString(s) => {
                write!(f, "Value(ManagedString({:?}))", unsafe { CStr::from_ptr(**s) })
            }
            Self::WideString(s) => write!(f, "Value(WideString({:?}))", s),
            Self::ManagedWideString(s) => {
                write!(f, "Value(ManagedWideString({:?}))", s)
            }
            Self::Object(o) => write!(f, "Value(Object({:?}))", o),
        }
    }
}

#[repr(C)]
pub struct GFxValue {
    /// FIXME: Unknown ptr lifetime
    pub objectInterface: *mut ObjectInterface,
    pub type_: ValueType,
    pad0C: u32,
    value: ValueUnion,
}
const _: () = assert!(core::mem::size_of::<GFxValue>() == 0x18);

impl Default for GFxValue {
    fn default() -> Self {
        Self {
            objectInterface: core::ptr::null_mut(),
            type_: Default::default(),
            pad0C: Default::default(),
            value: Default::default(),
        }
    }
}

impl GFxValue {
    #[inline]
    pub fn is_object(&self) -> bool {
        self.type_.contains(ValueType::Object | ValueType::Array | ValueType::DisplayObject)
    }

    #[inline]
    pub const fn is_number(&self) -> bool {
        self.type_.contains(ValueType::Number)
    }

    #[inline]
    pub fn is_display_object(&self) -> bool {
        self.type_ == ValueType::DisplayObject
    }

    pub fn get_value(&self) -> Option<Value> {
        let managed_string = ValueType::ManagedBit | ValueType::String;
        let managed_string_w = ValueType::ManagedBit | ValueType::StringW;

        unsafe {
            Some(match self.type_ {
                ValueType::Undefined => Value::Undefined,
                ValueType::Null => Value::Null,
                ValueType::Boolean => Value::Boolean(self.value.boolean),
                ValueType::Number => Value::Number(self.value.number),
                value if value == managed_string => Value::ManagedString(self.value.managedString),
                ValueType::String => Value::String(self.value.string),
                value if value == managed_string_w => {
                    Value::ManagedWideString(self.value.managedWideString)
                }
                ValueType::StringW => Value::WideString(self.value.wideString),
                ValueType::Object => Value::Object(self.value.obj),
                _ => return None,
            })
        }
    }

    pub fn get_member(&self, name: &CStr) -> Option<Self> {
        if !self.is_object() {
            return None;
        }

        let mut output = Self::default();
        unsafe {
            self.objectInterface.as_ref()?.get_member(
                self.value.obj,
                name.as_ptr(),
                &mut output,
                self.is_display_object(),
            );
        }

        Some(output)
    }
}

impl fmt::Debug for GFxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("GFxValue");

        debug_struct.field("objectInterface", &self.objectInterface);
        debug_struct.field("type_", &self.type_);

        let value = self.get_value();
        debug_struct.field("value", &value);

        debug_struct.finish()
    }
}
