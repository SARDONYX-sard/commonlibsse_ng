use crate::re::ActorValues::{ACTOR_VALUE_MODIFIER_CEnum, ActorValue};
use crate::re::BSFixedString::BSFixedString;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ACTOR_CRITICAL_STAGE {
    None = 0,
    GooStart = 1,
    GooEnd = 2,
    DisintegrateStart = 3,
    DisintegrateEnd = 4,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modifiers {
    pub modifiers: [f32; ACTOR_VALUE_MODIFIER_CEnum::count()],
}
const _: () = assert!(core::mem::size_of::<Modifiers>() == 0xC);

#[derive(Debug, Clone, PartialEq)]
pub struct ActorValueStorage {
    pub base_values: LocalMap<f32>,
    pub modifiers: LocalMap<Modifiers>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalMap<T> {
    pub actor_values: BSFixedString,
    pub entries: *mut T,
}

impl<T> LocalMap<T> {
    pub fn get(&self, actor_value: ActorValue) -> Option<&T> {
        let ak_vals = self.actor_values.as_bytes_with_null();

        if ak_vals.is_empty() && self.entries.is_null() {
            return None;
        }

        let mut idx = 0;
        while idx < ak_vals.len() && ak_vals[idx] != b'\0' {
            if ak_vals[idx] == actor_value as u8 {
                return unsafe { self.entries.add(idx).as_ref() };
            }
            idx += 1;
        }

        None
    }

    pub fn get_mut(&self, actor_value: ActorValue) -> Option<&mut T> {
        let ak_vals = self.actor_values.as_bytes_with_null();

        if ak_vals.is_empty() && self.entries.is_null() {
            return None;
        }

        let mut idx = 0;
        while idx < ak_vals.len() && ak_vals[idx] != b'\0' {
            if ak_vals[idx] == actor_value as u8 {
                return unsafe { self.entries.add(idx).as_mut() };
            }
            idx += 1;
        }

        None
    }
}

impl<T> core::ops::Index<ActorValue> for LocalMap<T> {
    type Output = T;

    fn index(&self, actor_value: ActorValue) -> &Self::Output {
        self.get(actor_value).expect("ActorValue not found")
    }
}

impl<T> core::ops::IndexMut<ActorValue> for LocalMap<T> {
    fn index_mut(&mut self, actor_value: ActorValue) -> &mut Self::Output {
        self.get_mut(actor_value).expect("ActorValue not found")
    }
}
