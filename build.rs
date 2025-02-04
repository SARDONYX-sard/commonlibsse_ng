// SPDX-FileCopyrightText: (C) 2024 metricexpansion
// SPDX-License-Identifier: CC-BY-NC-SA-4.0

fn main() {
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
    {
        let include_dir = crate_root.join("vcpkg_installed/x64-windows/include");
        bindgen(&include_dir);
    }
    println!("cargo:rustc-link-search=native={}", lib_path.display());
}

#[cfg(feature = "generate")]
fn bindgen<P>(include_dir: P)
where
    P: AsRef<std::path::Path>,
{
    let includes = include_dir.as_ref();

    let mut bindings = bindgen::Builder::default()
        // Fail calculation values
        // - vcpkg_installed\x64-windows\include\SKSE\Impl\Stubs.h:kInvalidPluginHandle = u32::MAX,
        // - vcpkg_installed\x64-windows\include\RE\G\GString.h:kFullFlag = 2147483648, (1 << 31)
        .allowlist_function("SKSE::AllocTrampoline")
        .allowlist_function("SKSE::GetMessagingInterface")
        .allowlist_function("SKSE::GetModCallbackEventSource")
        .allowlist_function("SKSE::GetPapyrusInterface")
        .allowlist_function("SKSE::GetSerializationInterface")
        .allowlist_function("SKSE::GetTrampoline")
        .allowlist_function("SKSE::Init")
        .allowlist_item("RE::.*")
        .allowlist_item("REL::.*")
        .allowlist_item("SKSE::.*")
        .allowlist_type("RE::AlchemyItem")
        .allowlist_type("RE::Calendar")
        .allowlist_type("RE::PlayerCharacter")
        .allowlist_type("RE::TESDataHandler")
        .allowlist_type("RE::TESObjectARMO")
        .allowlist_type("REL::Module")
        .allowlist_type("REL::Version")
        .allowlist_type("SKSE::LoadInterface")
        .allowlist_type("SKSE::PluginInfo")
        .allowlist_type("SKSE::PluginVersionData")
        .allowlist_type("SKSE::QueryInterface")
        .allowlist_type("SKSE::SerializationInterface")
        .allowlist_type("SKSE::Trampoline")
        .array_pointers_in_arguments(true)
        .array_pointers_in_arguments(true)
        .blocklist_function("RE::BSTSmallArrayHeapAllocator.*")
        .blocklist_function("RE::FxResponseArgsEx.*") // same link name -> crash
        .opaque_type("const_pointer")
        .opaque_type("difference_type")
        .opaque_type("pointer")
        .opaque_type("RE::BSTArray.*")
        .opaque_type("RE::BSTPointerAndFlags.*")
        .opaque_type("RE::BSTSingleton.*")
        .opaque_type("RE::BSTSmartPointer.*")
        .opaque_type("RE::NiT.*")
        .opaque_type("size_type")
        .opaque_type("SKSE::stl.*")
        .opaque_type("std::.*")
        //
        // Generator args
        .clang_arg("-D_CRT_USE_BUILTIN_OFFSETOF")
        .clang_arg("-DENABLE_COMMONLIBSSE_TESTING")
        .clang_arg("-DRUST_DEFINES")
        .clang_arg("-fdelayed-template-parsing")
        .clang_arg("-fms-compatibility")
        .clang_arg("-fms-extensions")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{includes}"))
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .enable_cxx_namespaces()
        .generate_inline_functions(true)
        .layout_tests(false)
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
        let string = String::from_utf8_lossy(&writer).replace("\r\n", "\n");
        std::fs::write(output, string.as_bytes()).unwrap();
    }
}

#[cfg(feature = "generate")]
const DEFINES: &[(&str, &str)] = &[
    ("ENABLE_SKYRIM_SE", "ENABLE_SKYRIM_SE"),
    // ("ENABLE_SKYRIM_AE", "ENABLE_SKYRIM_AE"),
    // ("ENABLE_SKYRIM_VR", "ENABLE_SKYRIM_VR"),
];

#[cfg(feature = "prebuilt")]
fn fetch_libs<P>(out_dir: P)
where
    P: AsRef<std::path::Path>,
{
    use std::io::Cursor;

    let url = "https://github.com/SARDONYX-sard/commonlibsse_ng/releases/download/push/CommonLibSSE-NG-prebuilt.zip";
    let out_dir = out_dir.as_ref();

    // Download zip(Wait up to 30 minutes to download 160 MB considering the slow network.)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60 * 30))
        .build()
        .unwrap();
    let response = client.get(url).send().expect("Failed to download ZIP");
    let bytes = response.bytes().expect("Failed to read response bytes");

    zip_extract::extract(Cursor::new(bytes), &out_dir, false).unwrap_or_else(|err| panic!("{err}"));
}
