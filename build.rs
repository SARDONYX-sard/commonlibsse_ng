// SPDX-FileCopyrightText: (C) 2024 metricexpansion
// SPDX-License-Identifier: MIT OR CC-BY-NC-SA-4.0
//
// See: https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/issues/2#note_2332635556

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lib_path = crate_root.join("vcpkg_installed/x64-windows/lib");

    if cfg!(all(feature = "vcpkg", feature = "prebuilt")) {
        panic!("Features `vcpkg` and `prebuilt` cannot be enabled at the same time.");
    }

    // Download C++ libraries
    let libs_existed = std::fs::exists(&lib_path).unwrap_or_default();
    if !libs_existed {
        #[cfg(feature = "prebuilt")]
        fetch_libs(&crate_root);

        #[cfg(feature = "vcpkg")]
        std::process::Command::new("vcpkg")
            .arg("install")
            .output()
            .expect("install by vcpkg");
    }

    #[cfg(feature = "generate")]
    bindgen(&crate_root);

    #[cfg(not(feature = "no_sys"))]
    {
        println!("cargo:rustc-link-search={}", lib_path.display());
        // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib
        println!("cargo:rustc-link-lib=static=CommonLibSSE");
        println!("cargo:rustc-link-lib=static=fmt");
        println!("cargo:rustc-link-lib=static=spdlog");
    }
    Ok(())
}

#[cfg(feature = "generate")]
fn bindgen<P>(crate_root: P)
where
    P: AsRef<std::path::Path>,
{
    let crate_root = crate_root.as_ref();
    let header = crate_root.join("wrapper.hpp");
    let include_dir = {
        let include_dir = crate_root.join("vcpkg_installed/x64-windows/include");
        include_dir.display().to_string()
    };

    let mut bindings = bindgen::Builder::default()
        // Fail calculation values
        // - vcpkg_installed\x64-windows\include\SKSE\Impl\Stubs.h:kInvalidPluginHandle = u32::MAX,
        // - vcpkg_installed\x64-windows\include\RE\G\GString.h:kFullFlag = 2147483648, (1 << 31)
        .allowlist_item("RE::.*")
        .allowlist_item("REL::.*")
        .allowlist_item("SKSE::.*")
        .blocklist_function("RE::BSTSmallArrayHeapAllocator.*") // rust-bindgen does not support generics.
        .blocklist_function("RE::FxResponseArgsEx.*") // The same `#[link_name = "<name>"]` is generated (e.g. `front`) and crashes, so stop generating it.
        .opaque_type("const_pointer") // It had to be an opaque type or it would have generated the wrong type.
        .opaque_type("difference_type") // It had to be an opaque type or it would have generated the wrong type.
        .opaque_type("pointer") // It had to be an opaque type or it would have generated the wrong type.
        .opaque_type("RE::BSTArray.*") // rust-bindgen does not support generics.
        .opaque_type("RE::BSTPointerAndFlags.*") // rust-bindgen does not support generics.
        .opaque_type("RE::BSTSingleton.*") //  rust-bindgen does not support generics.
        .opaque_type("RE::BSTSmartPointer.*") //  rust-bindgen does not support generics.
        .opaque_type("RE::NiT.*") //  rust-bindgen does not support generics.
        .opaque_type("size_type") // To avoid wrong type generation
        .opaque_type("SKSE::stl.*") // rust-bindgen does not support generics.
        .opaque_type("std::.*") // Cannot parse all C++ std
        //
        // Generator args(Somehow it errors if not in this order.)
        // MSCV compatibility: https://clang.llvm.org/docs/MSVCCompatibility.html
        .array_pointers_in_arguments(true)
        .header(header.display().to_string())
        .clang_arg("-D_CRT_USE_BUILTIN_OFFSETOF") // Ensure Clang uses its built-in offsetof for better compatibility with Windows code.
        .clang_arg("-DENABLE_COMMONLIBSSE_TESTING")
        .clang_arg("-std=c++20") // This is necessary because CommonLibSSE-NG depends on C++20.
        .clang_arg("-fms-compatibility") // Enable MSVC compatibility for MS-specific features (e.g., inline assembly).
        .clang_arg("-fms-extensions") // Allow MSVC-specific extensions like #pragma once and __declspec.
        .clang_arg("-fdelayed-template-parsing") // Delay template parsing to match MSVC's behavior.
        .clang_arg(format!("-I{include_dir}"))
        .default_enum_style(bindgen::EnumVariation::Rust {
            // By default, C enum is a single number, but since it is difficult to use and induces the type difference bug,
            // we will have it transformed into a Rust enum.
            non_exhaustive: false,
        })
        // .derive_default(true) // OFF: Because there is a bug that default is mistakenly impl if there is a single value in the enum.
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .enable_cxx_namespaces() // Have the C++ namespace reproduced in Rust for ease of use.
        // .generate_inline_functions(true) // The inline function cannot be called because it does not exist in the .lib file.
        // .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for (key, value) in DEFINES {
        bindings = bindings.clang_arg(format!("-D{key}={value}"));
    }

    let mut writer: Vec<u8> = Vec::new();
    let bindings = bindings.generate().expect("Unable to generate bindings");
    bindings
        .write(Box::new(&mut writer))
        .expect("Couldn't write bindings!");

    {
        let out_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let output = out_path.join("src/bindings.rs");
        let string = String::from_utf8_lossy(&writer)
            .replace("\r\n", "\n")
            // Fix incorrect `kInvalidPluginHandle` and `kFullFlag`(1 << 31) values.
            .replace(
                "kInvalidPluginHandle = -1",
                "kInvalidPluginHandle = u32::MAX",
            )
            .replace("kFullFlag = -9223372036854775808", "kFullFlag = 2147483648");
        std::fs::write(output, string.as_bytes()).unwrap();
    }
}

#[cfg(feature = "generate")]
const DEFINES: &[(&str, &str)] = &[
    ("ENABLE_SKYRIM_SE", "ON"),
    // ("ENABLE_SKYRIM_AE", "ON"),
    // ("ENABLE_SKYRIM_VR", "ON"),
];

#[cfg(feature = "prebuilt")]
fn fetch_libs<P>(out_dir: P)
where
    P: AsRef<std::path::Path>,
{
    use std::io::Cursor;

    let url = "https://github.com/SARDONYX-sard/commonlibsse_ng/releases/download/CommonLibSSE-NG-prebuilt/CommonLibSSE-NG-prebuilt.zip";
    let out_dir = out_dir.as_ref();

    // Download zip(Wait up to 30 minutes to download 160 MB considering the slow network.)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60 * 30))
        .build()
        .unwrap();
    let response = client.get(url).send().expect("Failed to download ZIP");
    let bytes = response.bytes().expect("Failed to read response bytes");

    zip_extract::extract(Cursor::new(bytes), out_dir, false).unwrap_or_else(|err| panic!("{err}"));
}
