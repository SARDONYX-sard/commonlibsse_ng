use core::ffi::{CStr, c_char};
use core::ptr;

use crate::re::Actor::Actor;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::GameSettingCollection::GameSettingCollection;
use crate::re::IMessageBoxCallback::IMessageBoxCallback;
use crate::re::INIPrefSettingCollection::INIPrefSettingCollection;
use crate::re::INISettingCollection::INISettingCollection;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::NiPoint3::NiPoint3;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::Setting::{Setting, SettingValue};
use crate::re::TESObjectREFR::TESObjectREFR;

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12204, ae_id = 12332)]
pub fn LookupReferenceByHandle_ActorImpl(
    handle: &RefHandle,
    refr_out: &mut NiPointer<Actor>,
) -> bool {
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12204, ae_id = 12332)]
pub fn LookupReferenceByHandle_RefrImpl(
    handle: *const RefHandle,
    refr_out: *mut NiPointer<TESObjectREFR>,
) -> bool {
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 51420, ae_id = 52269)]
pub fn CreateMessage(
    message: *const c_char,
    callback: *mut IMessageBoxCallback,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    button_text: *const c_char,
    secondary_button_text: *const c_char,
) {
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12193, ae_id = 12326)]
pub fn CreateRefHandle(handle_out: *mut RefHandle, ref_to: *mut TESObjectREFR) {}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 52050, ae_id = 52933)]
pub fn DebugNotification(
    notification: *const c_char,
    sound_to_play: *const c_char,
    cancel_if_already_queued: bool,
) {
}

pub fn DebugMessageBox(message: *const c_char) {
    unsafe {
        let ok = GameSettingCollection::get_singleton()
            .and_then(|gsc| gsc.__base.settings.__base.__base.__base.get(&c"sOk".as_ptr()))
            .and_then(|setting| {
                if let SettingValue::String(string) = setting.as_ref()?.get_value() {
                    return Some(string);
                }
                None
            })
            .unwrap_or(c"");
        CreateMessage(message, ptr::null_mut(), 0, 4, 10, ok.as_ptr(), ptr::null());
    }
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15779, ae_id = 16017)]
pub fn GetArmorFinalRating(
    armor_entry_data: *mut InventoryEntryData,
    armor_perks: f32,
    skill_multiplier: f32,
) -> f32 {
}

#[commonlibsse_ng_derive_internal::relocate(
    cast_as = "u32",
    default = "None",
    id(se = 523662, ae = 410201)
)]
pub fn GetDurationOfApplicationRunTime() -> Option<u32> {
    |as_type: AsType| Some(as_type)
}

pub fn GetINISetting(name: &CStr) -> Option<&'static Setting> {
    if let Some(setting) =
        INIPrefSettingCollection::get_singleton().and_then(|prefs| prefs.__base.get_setting(name))
    {
        return Some(setting);
    };

    INISettingCollection::get_singleton().and_then(|ini| ini.get_setting(name))
}

#[commonlibsse_ng_derive_internal::relocate(
    cast_as = "*mut f32",
    default = "None",
    id(se = 523660, ae = 410199)
)]
pub fn GetSecondsSinceLastFrame() -> Option<f32> {
    |as_type: AsType| unsafe { as_type.as_ref().copied() }
}

#[inline]
pub fn LookupReferenceByHandle_Actor(handle: &RefHandle, refr_out: &mut NiPointer<Actor>) -> bool {
    LookupReferenceByHandle_ActorImpl(handle, refr_out)
}

#[inline]
pub fn LookupReferenceByHandle_Refr(
    handle: &RefHandle,
    refr_out: &mut NiPointer<TESObjectREFR>,
) -> bool {
    LookupReferenceByHandle_RefrImpl(handle, refr_out)
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 52054, ae_id = 52939)]
pub fn PlaySound(editor_id: *const c_char) {}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 32275, ae_id = 33012)]
pub fn ShakeCamera(strength: f32, position: &NiPoint3, duration: f32) {}
