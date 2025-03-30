use core::ffi::c_void;

use crate::re::TESObjectREFR::{TESObjectREFR, TESObjectREFRVtbl};

#[repr(C)]
#[derive(Debug)]
pub struct Actor {
    pub __base: TESObjectREFR,
}

pub struct ActorVtbl {
    pub __base: TESObjectREFRVtbl,
    pub Unk_128: extern "C" fn(this: *mut Actor, c_void) -> c_void,
    pub Unk_129: extern "C" fn(this: *mut Actor, c_void) -> c_void,
}
