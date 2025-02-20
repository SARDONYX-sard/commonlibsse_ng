// dummy
pub mod Internal {
    pub struct VirtualMachine;

    impl VirtualMachine {
        pub const fn get_singleton() -> *mut Self {
            std::ptr::null_mut()
        }
    }
}
pub struct IVirtualMachine;
