// dummy
pub mod Internal {
    pub struct VirtualMachine;

    impl VirtualMachine {
        pub fn get_singleton() -> *mut VirtualMachine {
            std::ptr::null_mut()
        }
    }
}
pub struct IVirtualMachine;
