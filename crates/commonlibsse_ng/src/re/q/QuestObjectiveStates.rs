#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum QUEST_OBJECTIVE_STATE {
    Dormant = 0,
    Displayed = 1,
    Completed = 2,
    CompletedDisplayed = 3,
    Failed = 4,
    FailedDisplayed = 5,
}
