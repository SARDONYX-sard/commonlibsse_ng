use super::UserEvents::USER_EVENT_FLAG;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserEventEnabled {
    pub newUserEventFlag: USER_EVENT_FLAG, // 0x00
    pub oldUserEventFlag: USER_EVENT_FLAG, // 0x04
}
const _: () = assert!(core::mem::size_of::<UserEventEnabled>() == 0x8);
