use winnow::{
    ModalResult, Parser,
    ascii::{digit1, multispace0, space0},
    combinator::{delimited, opt, repeat, seq, terminated},
    token::take_until,
};

use crate::offsets_rtti_gen::hex;

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
            "pub const NiRTTI_{ident}: VariantID = VariantID::new({se_id}, {ae_id}, {vr_offset:#x});"
        )
    }
}

fn variant_id<'a>(input: &mut &'a str) -> ModalResult<VariantID<'a>> {
    seq! {
        VariantID {
            _: take_until(0.., "NiRTTI_"),
            _: "NiRTTI_",
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
    fn test_offset_id() {
        let input = r#"
            constexpr REL::VariantID NiRTTI_BGSAddonNodeSoundHandleExtra(514633, 400793, 0x2f8a838);
            constexpr REL::VariantID NiRTTI_BGSDecalNode(514417, 400564, 0x1f891a0);
            constexpr REL::VariantID NiRTTI_BSAnimGroupSequence(514462, 400606, 0x1f89358);
    "#;

        match variant_ids.parse(input) {
            Ok(variants) => {
                assert_eq!(
                    variants,
                    vec![
                        VariantID {
                            ident: "BGSAddonNodeSoundHandleExtra",
                            se_id: 514633,
                            ae_id: 400793,
                            vr_offset: 0x2f8a838,
                        },
                        VariantID {
                            ident: "BGSDecalNode",
                            se_id: 514417,
                            ae_id: 400564,
                            vr_offset: 0x1f891a0,
                        },
                        VariantID {
                            ident: "BSAnimGroupSequence",
                            se_id: 514462,
                            ae_id: 400606,
                            vr_offset: 0x1f89358,
                        },
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
        crate::offsets_ni_rtti_gen::generate_variant_ids(input, "./gen/offsets_ni_rtti.json")
            .unwrap_or_else(|err| panic!("{err}"));
    }

    fn gen_code(input: &str, file_name: &str) {
        let variants: Vec<VariantID> = serde_json::from_str(input).unwrap();
        let code = variants.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n");
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../crates/commonlibsse_ng/src/re")
            .join(file_name);

        let imports = "// C++ Original code
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
        let input = ::std::fs::read_to_string("./gen/offsets_ni_rtti.json").unwrap();
        gen_code(&input, "offsets_ni_rtti.rs");
    }
}
