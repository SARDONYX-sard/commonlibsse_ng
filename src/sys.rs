use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.hpp"
    safety!(unsafe)

    generate_ns!("RE::Calendar")
    // generate_ns!("SKSE")
}

// .allowlist_item("RE::.*")
// .allowlist_item("REL::.*")
// .allowlist_item("SKSE::.*")
// .blocklist_function("RE::BSTSmallArrayHeapAllocator.*") // rust-bindgen does not support generics.
// .blocklist_function("RE::FxResponseArgsEx.*") // The same `#[link_name = "<name>"]` is generated (e.g. `front`) and crashes, so stop generating it.
// .opaque_type("const_pointer") // It had to be an opaque type or it would have generated the wrong type.
// .opaque_type("difference_type") // It had to be an opaque type or it would have generated the wrong type.
// .opaque_type("pointer") // It had to be an opaque type or it would have generated the wrong type.
// .opaque_type("RE::BSTArray.*") // rust-bindgen does not support generics.
// .opaque_type("RE::BSTPointerAndFlags.*") // rust-bindgen does not support generics.
// .opaque_type("RE::BSTSingleton.*") //  rust-bindgen does not support generics.
// .opaque_type("RE::BSTSmartPointer.*") //  rust-bindgen does not support generics.
// .opaque_type("RE::NiT.*") //  rust-bindgen does not support generics.
// .opaque_type("size_type") // To avoid wrong type generation
// .opaque_type("SKSE::stl.*") // rust-bindgen does not support generics.
// .opaque_type("std::.*") // Cannot parse all C++ std
