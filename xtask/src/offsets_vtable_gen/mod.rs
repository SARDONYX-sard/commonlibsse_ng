mod variant_id;
mod variant_offset;

use crate::offsets_rtti_gen::VariantGenError;
use variant_id::{VariantIDTable, variant_id_table};
use variant_offset::{VariantOffsetTable, variant_offset_table};
use winnow::{
    ModalResult, Parser as _,
    ascii::multispace0,
    combinator::{delimited, opt, repeat},
    token::take_until,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct VTables<'a> {
    #[serde(bound(deserialize = "Vec<VariantIDTable<'a>>: serde::Deserialize<'de>"))]
    ids: Vec<VariantIDTable<'a>>,
    #[serde(bound(deserialize = "Vec<VariantOffsetTable<'a>>: serde::Deserialize<'de>"))]
    offsets: Vec<VariantOffsetTable<'a>>,
}

impl core::fmt::Display for VTables<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ids = self.ids.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n");
        let offsets =
            self.offsets.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n");

        let imports = "// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/RE/Offsets_VTABLE.h
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(non_upper_case_globals)]
use crate::rel::id::VariantID;
use crate::rel::offset::VariantOffset;";
        write!(f, "{imports}\n\n{}", [ids, offsets].join("\n"))
    }
}

fn variant_tables<'a>(input: &mut &'a str) -> ModalResult<VTables<'a>> {
    let _start = take_until(0.., "constexpr std::array").parse_next(input)?;

    let ids = repeat(1.., variant_id_table).parse_next(input)?;
    let _ = multispace0.parse_next(input)?;
    let offsets = repeat(1.., variant_offset_table).parse_next(input)?;

    let _end = delimited(multispace0, opt("}"), multispace0).parse_next(input)?;

    Ok(VTables { ids, offsets })
}

/// Generate variant ids & offsets json file from C++ code.
///
/// # Errors
/// If parse is failed, then return an error.
///
/// # Example
/// ```no_run
/// let input = include_str!("D:/Programming/cpp/CommonLibVR/include/RE/Offsets_VTABLE.h");
/// xtask::offsets_vtable_gen::generate_variant_vtables(input, "./gen/offsets_vtable.json")
///     .unwrap_or_else(|err| panic!("{err}"));
/// ```
pub fn generate_variant_vtables(
    input: &str,
    output: impl AsRef<std::path::Path>,
) -> Result<(), VariantGenError> {
    let output = output.as_ref();

    let variant_tables = variant_tables
        .parse(input)
        .map_err(|err| VariantGenError::Parse { err: err.to_string() })?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&variant_tables)?;
    std::fs::write(output, &json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{variant_id::VariantID, variant_offset::VariantOffset, *};

    #[test]
    fn test_variant_tables() {
        let input = r#"
        constexpr std::array<REL::VariantID, 1>  VTABLE_ConcreteFormFactory_AlchemyItem_46_{ REL::VariantID(228356, 186192, 0x1596bc0) };
        constexpr std::array<REL::VariantID, 1>  VTABLE_ConcreteObjectFormFactory_AlchemyItem_46_17_2_{ REL::VariantID(228357, 186194, 0x1596c00) };
        constexpr std::array<REL::VariantID, 1>  VTABLE_IFormFactory{ REL::VariantID(228345, 186197, 0x1596628) };
        constexpr std::array<REL::VariantID, 4>  VTABLE_EffectSetting{ REL::VariantID(228544, 186368, 0x1598558), REL::VariantID(228545, 186370, 0x1598738), REL::VariantID(228546, 186372, 0x1598770), REL::VariantID(228547, 186374, 0x15987a0) };

        constexpr std::array<REL::VariantOffset, 1> VTABLE_std___Ref_count_obj_BSLocklessSimpleList_ActiveEffect_____Node_{ REL::VariantOffset(0, 0, 0x15a4358) };
    "#;

        match variant_tables.parse(input) {
            Ok(variants) => assert_eq!(
                variants,
                VTables {
                    ids: vec![
                        VariantIDTable {
                            ident: "ConcreteFormFactory_AlchemyItem_46_",
                            len: 1,
                            variants: vec![VariantID {
                                se_id: 228356,
                                ae_id: 186192,
                                vr_offset: 0x1596bc0
                            }],
                        },
                        VariantIDTable {
                            ident: "ConcreteObjectFormFactory_AlchemyItem_46_17_2_",
                            len: 1,
                            variants: vec![VariantID {
                                se_id: 228357,
                                ae_id: 186194,
                                vr_offset: 0x1596c00
                            }],
                        },
                        VariantIDTable {
                            ident: "IFormFactory",
                            len: 1,
                            variants: vec![VariantID {
                                se_id: 228345,
                                ae_id: 186197,
                                vr_offset: 0x1596628
                            }]
                        },
                        VariantIDTable {
                            ident: "EffectSetting",
                            len: 4,
                            variants: vec![
                                VariantID { se_id: 228544, ae_id: 186368, vr_offset: 0x1598558 },
                                VariantID { se_id: 228545, ae_id: 186370, vr_offset: 0x1598738 },
                                VariantID { se_id: 228546, ae_id: 186372, vr_offset: 0x1598770 },
                                VariantID { se_id: 228547, ae_id: 186374, vr_offset: 0x15987a0 }
                            ]
                        }
                    ],
                    offsets: vec![VariantOffsetTable {
                        ident: "std___Ref_count_obj_BSLocklessSimpleList_ActiveEffect_____Node_",
                        len: 1,
                        variants: vec![VariantOffset {
                            se_offset: 0,
                            ae_offset: 0,
                            vr_offset: 0x15a4358
                        }],
                    }]
                }
            ),
            Err(err) => panic!("Error:\n{err}"),
        }
    }

    #[ignore = "need C++ src (from generate manually)"]
    #[test]
    fn test_de_vtable() {
        let input = include_str!("D:/Programming/cpp/CommonLibVR/include/RE/Offsets_VTABLE.h");
        crate::offsets_vtable_gen::generate_variant_vtables(input, "./gen/offsets_vtable.json")
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[ignore = "need offsets_vtable.json (from generate manually)"]
    #[test]
    fn test_gen_code() {
        let input = ::std::fs::read_to_string("./gen/offsets_vtable.json").unwrap();
        let vtables: VTables = serde_json::from_str(&input).unwrap();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../crates/commonlibsse_ng/src/re")
            .join("offsets_vtable.rs");
        std::fs::write(path, format!("{vtables}\n")).unwrap();
    }
}
