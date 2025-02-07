use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.hpp"
    // generate_ns!("SKSE")
    generate_ns!("REL")
    // generate_ns!("RE")
}

// safety!(unsafe)
