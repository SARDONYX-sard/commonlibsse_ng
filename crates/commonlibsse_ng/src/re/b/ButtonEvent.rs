use core::alloc::Layout;
use core::ptr;

use crate::re::BSFixedString::BSFixedString;
use crate::re::IDEvent::{IDEvent, IDEventVtbl};
use crate::re::InputDevices::INPUT_DEVICE;
use crate::re::InputEvent::{INPUT_EVENT_TYPE, InputEvent};
use crate::re::MemoryManager::alloc::alloc_zeroed;
use crate::re::TESBox::TESBox;
use crate::re::offsets_rtti::RTTI_ButtonEvent;
use crate::re::offsets_vtable::VTABLE_ButtonEvent;
use crate::rel::ResolvableAddress;
use crate::rel::id::{DataBaseError, VariantID};
use crate::rel::module::is_vr;
use crate::rel::relocation::{RelocationError, relocate_member, relocate_member_mut};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ButtonEvent {
    pub __base: IDEvent, // 0x00
}
const _: () = assert!(core::mem::size_of::<ButtonEvent>() == 0x28);

impl ButtonEvent {
    /// Address & offset of RTTI for `ButtonEvent`.
    pub const RTTI: VariantID = RTTI_ButtonEvent;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ButtonEvent;

    /// # Errors
    pub fn vtable() -> Result<&'static ButtonEventVtbl, DataBaseError> {
        Self::VTABLE[0].address().map(|vtable| unsafe { vtable.cast().as_ref() })
    }

    /// # Errors
    ///
    /// # Panics
    #[allow(clippy::unwrap_in_result)]
    pub fn new_boxed(
        input_device: INPUT_DEVICE,
        user_event: BSFixedString,
        id_code: u32,
        value: f32,
        held_down_secs: f32,
    ) -> Result<TESBox<Self>, DataBaseError> {
        const VR_BUTTON_EVENT_SIZE: usize = 0x30;
        const TOTAL_SIZE: usize = VR_BUTTON_EVENT_SIZE + size_of::<RUNTIME_DATA>();

        let layout = Layout::from_size_align(TOTAL_SIZE, align_of::<Self>()).expect("Valid layout");

        unsafe {
            let ptr = alloc_zeroed(layout).cast::<u8>();
            if ptr.is_null() {
                #[cfg(feature = "tracing")]
                tracing::error!("Heap allocation failed");
                return Err(DataBaseError::Poisoned);
            }

            // Set up the vtable
            let vtable = Self::vtable()? as *const ButtonEventVtbl;

            // Construct ButtonEvent in place
            let event_ptr = ptr.cast::<Self>();
            ptr::write(
                event_ptr,
                Self {
                    __base: IDEvent {
                        __base: InputEvent {
                            vtable: vtable.cast(),
                            device: input_device,
                            eventType: INPUT_EVENT_TYPE::Button,
                            next: None,
                        },
                        userEvent: user_event,
                        idCode: id_code,
                        pad24: 0,
                    },
                },
            );

            // Initialize runtime data
            {
                const SE_BUTTON_EVENT_SIZE: usize = 0x28;
                let runtime_offset =
                    if is_vr() { SE_BUTTON_EVENT_SIZE } else { VR_BUTTON_EVENT_SIZE };
                let runtime_ptr = ptr.add(runtime_offset).cast::<RUNTIME_DATA>();
                ptr::write(runtime_ptr, RUNTIME_DATA { value, heldDownSecs: held_down_secs });
            }

            Ok(TESBox::from_raw(event_ptr))
        }
    }

    /// Gets fields whose offset is determined at runtime.
    ///
    /// # Errors
    /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
    /// Specifically, it calls `ModuleState::map_active`, which could result in an error.
    #[inline]
    pub fn get_runtime_data(&self) -> Result<&RUNTIME_DATA, RelocationError> {
        relocate_member(self, 0x28, 0x30)
    }

    /// Gets mutable fields whose offset is determined at runtime.
    ///
    /// # Errors
    /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
    /// Specifically, it calls `ModuleState::map_active_mut`, which could result in an error.
    #[inline]
    pub fn get_runtime_data_mut(&mut self) -> Result<&mut RUNTIME_DATA, RelocationError> {
        relocate_member_mut(self, 0x28, 0x30)
    }

    #[inline]
    pub fn value(&self) -> Option<f32> {
        Some(self.get_runtime_data().ok()?.value)
    }

    #[inline]
    pub fn held_duration(&self) -> Option<f32> {
        Some(self.get_runtime_data().ok()?.heldDownSecs)
    }

    #[inline]
    pub fn is_pressed(&self) -> bool {
        match self.value() {
            Some(value) => value > 0.0,
            None => false,
        }
    }

    #[inline]
    pub fn is_repeating(&self) -> bool {
        match self.held_duration() {
            Some(value) => value > 0.0,
            None => false,
        }
    }

    #[inline]
    pub fn is_down(&self) -> bool {
        self.is_pressed() && self.held_duration().is_some_and(|duration| duration > 0.0)
    }

    #[inline]
    pub fn is_held(&self) -> bool {
        self.is_pressed() && self.is_repeating()
    }

    #[inline]
    pub fn is_up(&self) -> bool {
        self.value().is_some_and(|value| value > 0.0) && self.is_repeating()
    }
}

pub struct ButtonEventVtbl {
    pub __base: IDEventVtbl, // 0x00
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RUNTIME_DATA {
    pub value: f32,
    pub heldDownSecs: f32,
}
const _: () = assert!(core::mem::size_of::<RUNTIME_DATA>() == 0x8);
