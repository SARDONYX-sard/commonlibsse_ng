/// SPDX-FileCopyrightText: (C) 2024 metricexpansion
/// SPDX-License-Identifier: MIT OR CC-BY-NC-SA-4.0
///
/// See: https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/issues/2#note_2332635556

fn main() -> miette::Result<()> {
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
    bindgen(&crate_root)?;

    // #[cfg(not(feature = "no_sys"))]
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
fn bindgen<P>(crate_root: P) -> miette::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let crate_root = crate_root.as_ref();
    let header = crate_root.to_path_buf();
    let include_dir = {
        let include_dir = crate_root.join("vcpkg_installed/x64-windows/include");
        include_dir
    };

    let mut b = autocxx_build::Builder::new("src/sys.rs", &[&header, &include_dir])
        .extra_clang_args(&[
            "-std=c++20",
            "-D_CRT_USE_BUILTIN_OFFSETOF", // Ensure Clang uses its built-in offsetof for better compatibility with Windows code.
            "-DENABLE_COMMONLIBSSE_TESTING",
            "-DENABLE_SKYRIM_SE",
            "-fms-compatibility", // Enable MSVC compatibility for MS-specific features (e.g., inline assembly).
            "-fms-extensions", // Allow MSVC-specific extensions like #pragma once and __declspec.
        ])
        // .custom_gendir(crate_root.join("src").join("sys"))
        .build()?;
    b.opt_level(2)
        .cpp(true)
        .flag_if_supported("-std=c++20")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("-D_CRT_USE_BUILTIN_OFFSETOF")
        .flag_if_supported("-DENABLE_COMMONLIBSSE_TESTING")
        .flag_if_supported("-DENABLE_SKYRIM_SE")
        .flag_if_supported("-fms-compatibility")
        .flag_if_supported("-fms-extensions")
        .compile("commonlibsse_ng"); // arbitrary library name, pick anything

    println!("cargo:rerun-if-changed=src/sys.rs");

    Ok(())
}

// #[cfg(feature = "generate")]
// const DEFINES: &[(&str, &str)] = &[
//     ("ENABLE_SKYRIM_SE", "ON"),
//     // ("ENABLE_SKYRIM_AE", "ON"),
//     // ("ENABLE_SKYRIM_VR", "ON"),
// ];

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

    zip_extract::extract(Cursor::new(bytes), &out_dir, false).unwrap_or_else(|err| panic!("{err}"));
}
