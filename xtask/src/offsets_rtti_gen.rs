use winnow::{
    ModalResult, Parser,
    ascii::{digit1, hex_digit1, multispace0, space0},
    combinator::{delimited, opt, preceded, repeat, seq, terminated},
    token::take_until,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct VariantID<'a> {
    ident: &'a str,
    se_id: u32,
    ae_id: u32,
    vr_offset: u64,
}

impl core::fmt::Display for VariantID<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { ident, se_id, ae_id, vr_offset } = self;
        write!(
            f,
            "pub const RTTI_{ident}: VariantID = VariantID::new({se_id}, {ae_id}, {vr_offset:#x});"
        )
    }
}

pub fn hex(input: &mut &str) -> ModalResult<u64> {
    preceded("0x", hex_digit1).try_map(|hex| u64::from_str_radix(hex, 16)).parse_next(input)
}

fn variant_id<'a>(input: &mut &'a str) -> ModalResult<VariantID<'a>> {
    seq! {
        VariantID {
            _: take_until(0.., "RTTI_"),
            _: "RTTI_",
            ident: terminated(take_until(0.., "("), "("), // ident after `RTTI_`  (e.g.: `ConcreteFormFactory_AlchemyItem_46_`)
            _: space0,
            se_id: terminated(digit1, ", ").parse_to(),
            ae_id: terminated(digit1, ", ").parse_to(),
            vr_offset: hex,
            _: ");",
        }
    }
    .parse_next(input)
}

fn variant_ids<'a>(input: &mut &'a str) -> ModalResult<Vec<VariantID<'a>>> {
    let namespace_end = opt(("}", multispace0));
    repeat(0.., delimited(multispace0, variant_id, (multispace0, namespace_end))).parse_next(input)
}

/// Generate variant RTTI ids json file from C++ code.
///
/// # Errors
/// If parse is failed, then return an error.
///
/// # Example
/// ```no_run
/// let input = include_str!("D:/Programming/cpp/CommonLibVR/include/RE/Offsets_RTTI.h");
/// xtask::offsets_rtti_gen::generate_variant_ids(input, "./offsets_rtti.json").unwrap_or_else(|err| panic!("{err}"));
/// ```
pub fn generate_variant_ids(
    input: &str,
    output: impl AsRef<std::path::Path>,
) -> Result<(), VariantGenError> {
    let output = output.as_ref();

    let variant_ids =
        variant_ids.parse(input).map_err(|err| VariantGenError::Parse { err: err.to_string() })?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&variant_ids)?;
    std::fs::write(output, &json)?;
    Ok(())
}

/// Generation error
#[derive(Debug, snafu::Snafu)]
pub enum VariantGenError {
    /// Error parsing input
    #[snafu(display("Error parsing input:\n {err}"))]
    Parse { err: String },

    /// Inherited std::io error
    #[snafu(transparent)]
    Io { source: std::io::Error },

    /// Inherited serde_json error
    #[snafu(transparent)]
    Json { source: serde_json::Error },
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq; // To debug

    #[test]
    fn test_hex() {
        let input = "0x12345678";
        match hex.parse(input) {
            Ok(number) => assert_eq!(number, 0x12345678),
            Err(err) => panic!("Error: \n{err}"),
        }
    }

    #[test]
    fn test_offset_id() {
        let input = r#"constexpr REL::VariantID RTTI_ConcreteFormFactory_AlchemyItem_46_(684591, 392213, 0x1ed7000);"#;
        match variant_id.parse(input) {
            Ok(variant) => {
                assert_eq!(
                    variant,
                    VariantID {
                        ident: "ConcreteFormFactory_AlchemyItem_46_",
                        se_id: 684591,
                        ae_id: 392213,
                        vr_offset: 0x1ed7000,
                    }
                );
            }
            Err(err) => panic!("Error: \n{err}"),
        }
    }

    #[test]
    fn main() {
        let input = r#"
        constexpr REL::VariantID RTTI_ConcreteFormFactory_AlchemyItem_46_(684591, 392213, 0x1ed7000);
        constexpr REL::VariantID RTTI_IFormFactory(684588, 392214, 0x1ed6cf8);
        constexpr REL::VariantID RTTI_ConcreteObjectFormFactory_AlchemyItem_46_17_2_(684590, 392212, 0x1ed6fb0);
        constexpr REL::VariantID RTTI_BaseFormComponent(513847, 392215, 0x1ed6cd0);
    "#;

        match variant_ids.parse(input) {
            Ok(variants) => {
                assert_eq!(
                    variants,
                    vec![
                        VariantID {
                            ident: "ConcreteFormFactory_AlchemyItem_46_",
                            se_id: 684591,
                            ae_id: 392213,
                            vr_offset: 0x1ed7000,
                        },
                        VariantID {
                            ident: "IFormFactory",
                            se_id: 684588,
                            ae_id: 392214,
                            vr_offset: 0x1ed6cf8,
                        },
                        VariantID {
                            ident: "ConcreteObjectFormFactory_AlchemyItem_46_17_2_",
                            se_id: 684590,
                            ae_id: 392212,
                            vr_offset: 0x1ed6fb0,
                        },
                        VariantID {
                            ident: "BaseFormComponent",
                            se_id: 513847,
                            ae_id: 392215,
                            vr_offset: 0x1ed6cd0,
                        }
                    ]
                );
            }
            Err(err) => panic!("Error: \n{err}"),
        }
    }

    #[ignore = "need C++ src (from generate manually)"]
    #[test]
    fn test_de_ni_rtti() {
        let input = include_str!("D:/Programming/cpp/CommonLibVR/include/RE/Offsets_NiRTTI.h");
        crate::offsets_rtti_gen::generate_variant_ids(input, "./offsets_ni_rtti.json")
            .unwrap_or_else(|err| panic!("{err}"));
    }

    fn gen_code(input: &str, file_name: &str) {
        let variants: Vec<VariantID> = serde_json::from_str(input).unwrap();
        let code = variants.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n");
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../crates/commonlibsse_ng/src/re")
            .join(file_name);

        let imports = "// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/RE/Offsets_NiRTTI.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/RE/Offsets_RTTI.h
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(non_upper_case_globals)]
use crate::rel::id::VariantID;";
        std::fs::write(path, format!("{imports}\n\n{code}\n")).unwrap();
    }

    #[ignore = "need offsets_ni_rtti.json (from generate manually)"]
    #[test]
    fn test_gen_code() {
        let input = ::std::fs::read_to_string("./offsets_rtti.json").unwrap();
        gen_code(&input, "offsets_rtti.rs");

        let input = ::std::fs::read_to_string("./offsets_ni_rtti.json").unwrap();
        gen_code(&input, "offsets_ni_rtti.rs");
    }
}
