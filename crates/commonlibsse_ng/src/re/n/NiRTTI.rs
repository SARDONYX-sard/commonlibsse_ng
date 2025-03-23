use core::ffi::c_char;

#[repr(C)]
pub struct NiRTTI {
    name: *const c_char,
    base_rtti: *const NiRTTI,
}
const _: () = assert!(std::mem::size_of::<NiRTTI>() == 16);

impl NiRTTI {
    #[inline]
    pub const fn get_name(&self) -> *const c_char {
        self.name
    }

    #[inline]
    pub const fn get_base_rtti(&self) -> *const Self {
        self.base_rtti
    }

    #[inline]
    pub fn is_kind_of(&self, rtti: *const Self) -> bool {
        let mut iter = self as *const Self;
        while !iter.is_null() {
            if iter == rtti {
                return true;
            }
            iter = unsafe { (*iter).get_base_rtti() };
        }
        false
    }
}

// // Downcast equivalent
// pub fn netimmerse_cast<To, From>(a_from: *const From) -> Option<*const To>
// where
//     To: 'static,
//     From: 'static,
// {
//     if a_from.is_null() {
//         return None;
//     }

//     let to_rtti: *const NiRTTI = &To::Ni_RTTI; // This assumes you have a `Ni_RTTI` defined for each type.
//     let mut from_rtti = unsafe { (*a_from).get_rtti() };

//     while !from_rtti.is_null() {
//         if from_rtti == to_rtti {
//             return Some(a_from as *const To);
//         }
//         from_rtti = unsafe { (*from_rtti).get_base_rtti() };
//     }

//     None
// }
