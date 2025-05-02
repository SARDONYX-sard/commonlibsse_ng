use crate::re::BSFixedString::BSFixedString;
use crate::re::BSInputDeviceManager::BSInputDeviceManager;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTEvent::BSTEventSource;
use crate::re::InputDevices::{INPUT_DEVICE, INPUT_DEVICE_VR_CEnum};
use crate::re::PCGamepadType::PC_GAMEPAD_TYPE_CEnum;
use crate::re::UserEventEnabled::UserEventEnabled;
use crate::re::UserEvents::{INPUT_CONTEXT_ID, INPUT_CONTEXT_ID_VR_CEnum, USER_EVENT_FLAG};
use crate::rel::module::ModuleState;
use crate::rel::relocation::{RelocationError, raw_pointer_as_mut, raw_pointer_as_ref};
use crate::skse::version::RUNTIME_SSE_1_6_1130;

#[repr(C)]
#[derive(Debug)]
pub struct ControlMap {
    pub __base: [u8; 1], // 0x000: BSTSingletonSDM<ControlMap> address
    pub __base1: BSTEventSource<UserEventEnabled>, // 0x008: vtable size
    pub controlMap: [*mut InputContext; INPUT_CONTEXT_ID_VR_CEnum::count()], // 0x060
}
const _: () = assert!(core::mem::size_of::<ControlMap>() == 0xE8);

impl ControlMap {
    /// Gets the singleton instance of `ControlMap`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut ControlMap",
        default = "None",
        deref_once,
        id(se = 514705, ae = 400863)
    )]
    pub fn get_singleton() -> Option<&'static ControlMap> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Gets the mutable singleton instance of `ControlMap`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut ControlMap",
        default = "None",
        deref_once,
        id(se = 514705, ae = 400863)
    )]
    pub fn get_singleton_mut() -> Option<&'static mut ControlMap> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }

    /// Gets fields whose offset is determined at runtime.
    ///
    /// # Errors
    /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
    /// Specifically, it calls `ModuleState::map_active`, which could result in an error.
    pub fn get_runtime_data(&self) -> Result<&RUNTIME_DATA, RelocationError> {
        let (is_vr, is_ae_1_6_1130) = ModuleState::map_or_init(|module| {
            (module.runtime.is_vr(), module.version >= RUNTIME_SSE_1_6_1130)
        })?;

        let this = self as *const Self;
        let member_ptr = if is_ae_1_6_1130 {
            this.wrapping_offset(0xf0).cast()
        } else {
            let offset = if is_vr { 0xE8 } else { 0x108 };
            this.wrapping_offset(offset).cast()
        };

        Ok(unsafe { raw_pointer_as_ref(member_ptr) }?)
    }

    /// Gets mutable fields whose offset is determined at runtime.
    ///
    /// # Errors
    /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
    /// Specifically, it calls `ModuleState::map_active_mut`, which could result in an error.
    pub fn get_runtime_data_mut(&mut self) -> Result<&mut RUNTIME_DATA, RelocationError> {
        let (is_vr, is_ae_1_6_1130) = ModuleState::map_or_init(|module| {
            (module.runtime.is_vr(), module.version >= RUNTIME_SSE_1_6_1130)
        })?;

        let this = self as *mut Self;

        let member_ptr = if is_ae_1_6_1130 {
            this.wrapping_offset(0xf0).cast()
        } else {
            let offset = if is_vr { 0xE8 } else { 0x108 };
            this.wrapping_offset(offset).cast()
        };

        Ok(unsafe { raw_pointer_as_mut(member_ptr) }?)
    }

    pub fn allow_text_input(&mut self, allow: bool) -> Option<i8> {
        let text_entry_count = &mut (self.get_runtime_data_mut().ok()?.textEntryCount);

        if allow {
            if *text_entry_count != -1 {
                *text_entry_count += 1;
            }
        } else if *text_entry_count != 0 {
            *text_entry_count -= 1;
        }

        Some(*text_entry_count)
    }

    pub fn get_button_name_from_user_event(
        &self,
        event_id: &BSFixedString,
        device: INPUT_DEVICE,
    ) -> Option<BSFixedString> {
        for input_context in self.controlMap {
            let Some(input_context) = (unsafe { input_context.as_ref() }) else {
                continue;
            };

            let device_mappings = input_context.deviceMappings.get(device.0 as usize)?;

            for mapping in device_mappings {
                if mapping.eventID != *event_id {
                    continue;
                }

                let input_key = mapping.inputKey;
                if input_key == 0xFF {
                    break;
                }

                let input_device_manager = BSInputDeviceManager::get_singleton_mut()?;
                if let Some(output) =
                    input_device_manager.get_button_name_from_id(device, input_key as u32)
                {
                    return Some(output);
                };
            }
        }

        None
    }

    pub fn get_user_event_name(
        &self,
        button_id: u32,
        device: INPUT_DEVICE,
        context: INPUT_CONTEXT_ID,
    ) -> Option<&BSFixedString> {
        let input_ctx = self.controlMap.get(context.0 as usize)?;
        let mappings = unsafe { input_ctx.as_ref() }?.deviceMappings.get(device.0 as usize)?;

        // Instead of C++ `equal_range`
        let search_target_key = button_id as u16;
        let slice = mappings.as_slice();
        let start = slice.partition_point(|x| x.inputKey < search_target_key);
        let end = slice.partition_point(|x| x.inputKey <= search_target_key);

        if end - start == 1 { Some(&slice[start].eventID) } else { None }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67244, ae_id = 68544)]
    #[inline]
    pub fn pop_input_context(&mut self, context: INPUT_CONTEXT_ID) -> bool {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67243, ae_id = 68543)]
    #[inline]
    pub fn push_input_context(&mut self, context: INPUT_CONTEXT_ID) -> bool {}
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RUNTIME_DATA {
    pub linkedMappings: BSTArray<LinkedMapping>, // 0x0E8, VR: 0x108
    pub contextPriorityStack: BSTArray<INPUT_CONTEXT_ID>, // 0x100, VR: 0x120
    pub enabledControls: USER_EVENT_FLAG,        // 0x118, VR: 0x138
    pub unk11C: USER_EVENT_FLAG,                 // 0x11C, VR: 0x13C
    pub textEntryCount: i8,                      // 0x120, VR: 0x140
    pub ignoreKeyboardMouse: bool,               // 0x121, VR: 0x141
    pub ignoreActivateDisabledEvents: bool,      // 0x122, VR: 0x142
    pub pad123: u8,                              // 0x123, VR: 0x143
    pub gamePadMapType: PC_GAMEPAD_TYPE_CEnum,   // 0x124, VR: 0x144
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct UserEventMapping {
    pub eventID: BSFixedString,              // 0x00
    pub inputKey: u16,                       // 0x08
    pub modifier: u16,                       // 0x08
    pub indexInContext: i8,                  // 0x0C
    pub remappable: bool,                    // 0x0D
    pub linked: bool,                        // 0x0E
    pub userEventGroupFlag: USER_EVENT_FLAG, // 0x10
    pub pad14: u32,                          // 0x14
}
const _: () = assert!(core::mem::size_of::<UserEventMapping>() == 0x18);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputContext {
    pub deviceMappings: [BSTArray<UserEventMapping>; INPUT_DEVICE_VR_CEnum::count()], // 0x10
}
const _: () = assert!(core::mem::size_of::<InputContext>() == 0xF0);

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct LinkedMapping {
    pub linkedMappingName: BSFixedString,       // 0x00
    pub linkedMappingContext: INPUT_CONTEXT_ID, // 0x08
    pub device: INPUT_DEVICE,                   // 0x0C
    pub linkFromContext: INPUT_CONTEXT_ID,      // 0x10
    pub pad14: u32,                             // 0x14
    pub linkFromName: BSFixedString,            // 0x18
}
const _: () = assert!(core::mem::size_of::<LinkedMapping>() == 0x20);
