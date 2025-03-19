macro_rules! define_transparent_type {
    ($(#[$id_docs:meta])* $name:ident($type:ty) $(#[$new_docs:meta])*, $(#[$get_docs:meta])*) => {
        $(#[$id_docs])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name($type);

        impl $name {
            $(#[$new_docs])*
            #[inline]
            pub const fn new(value: $type) -> Self {
                Self(value)
            }

            $(#[$get_docs])*
            #[inline]
            pub const fn get(self) -> $type {
                self.0
            }
        }
    };
}

define_transparent_type!(
    /// A transparent wrapper for `FormID` (equivalent to `std::uint32_t` in C++).
    FormID(u32)
    /// Creates a new `FormID` from a `u32` value.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::FormID;
    /// let form_id = FormID::new(12345);
    /// ```
,
    /// Returns the contained value as a primitive type.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::FormID;
    /// let form_id = FormID::new(12345);
    /// assert_eq!(form_id.get(), 12345);
    /// ```
);

define_transparent_type!(
    /// A transparent wrapper for `RefHandle` (equivalent to `std::uint32_t` in C++).
    RefHandle(u32)
    /// Creates a new `RefHandle` from a `u32` value.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::RefHandle;
    /// let ref_handle = RefHandle::new(98765);
    /// ```
,
    /// Returns the contained value as a `u32` type.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::RefHandle;
    /// let ref_handle = RefHandle::new(98765);
    /// assert_eq!(ref_handle.get(), 98765);
    /// ```
);

define_transparent_type!(
    /// A transparent wrapper for `VMHandle` (equivalent to `std::uint64_t` in C++).
    VMHandle(u64)
    /// Creates a new `VMHandle` from a `u64` value.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::VMHandle;
    /// let vm_handle = RE::VMHandle::new(1234567890123456);
    /// ```
,
    /// Returns the contained value as a `u64` type.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::VMHandle;
    /// let vm_handle = VMHandle::new(1234567890123456);
    /// assert_eq!(vm_handle.get(), 1234567890123456);
    /// ```
);

define_transparent_type!(
    /// A transparent wrapper for `VMStackID` (equivalent to `std::uint32_t` in C++).
    VMStackID(u32)
    /// Creates a new `RefHandle` from a `u32` value.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::VMStackID;
    /// let ref_handle = VMStackID::new(1234);
    /// ```
,
    /// Returns the contained value as a `u32` type.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::VMStackID;
    /// let vm_stack_id = VMStackID::new(1234);
    /// assert_eq!(vm_stack_id.get(), 1234);
    /// ```
);

define_transparent_type!(
    /// A transparent wrapper for `VMTypeID` (equivalent to `std::uint32_t` in C++).
    VMTypeID(u32)
    /// Creates a new `VMTypeID` from a `u32` value.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::VMTypeID;
    /// let vm_type_id = VMTypeID::new(5678);
    /// ```
,
    /// Returns the contained value as a `u32` type.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSCoreTypes::VMTypeID;
    /// let vm_type_id = VMTypeID::new(5678);
    /// assert_eq!(vm_type_id.get(), 5678);
    /// ```
);
