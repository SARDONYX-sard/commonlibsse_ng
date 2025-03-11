use winnow::{
    ModalResult, Parser,
    ascii::{digit1, newline, space0},
    combinator::{opt, preceded, repeat, seq, terminated},
    token::take_until,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariantOffset {
    pub se_offset: u32,
    pub ae_offset: u32,
    pub vr_offset: u64,
}

impl core::fmt::Display for VariantOffset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { se_offset, ae_offset, vr_offset } = *self;
        write!(f, "VariantOffset::new({se_offset}, {ae_offset}, {vr_offset:#x})")
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariantOffsetTable<'a> {
    pub ident: &'a str,
    pub len: usize,
    pub variants: Vec<VariantOffset>,
}

impl core::fmt::Display for VariantOffsetTable<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { ident, len, variants } = self;
        write!(
            f,
            "pub const VTABLE_{ident}: [VariantOffset; {len}] = [{}];",
            variants.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")
        )
    }
}

fn variant_offset(input: &mut &str) -> ModalResult<VariantOffset> {
    seq! {
        VariantOffset {
            _: "REL::VariantOffset(",
            se_offset: terminated(digit1, ", ").parse_to(),
            ae_offset: terminated(digit1, ", ").parse_to(),
            vr_offset: crate::offsets_rtti_gen::hex,
            _: ")",
        }
    }
    .parse_next(input)
}

pub fn variant_offset_table<'a>(input: &mut &'a str) -> ModalResult<VariantOffsetTable<'a>> {
    seq! {
        VariantOffsetTable {
            _: space0,
            _: "constexpr std::array<REL::VariantOffset, ",
            len: terminated(digit1.parse_to::<usize>(), ">"),
            _: preceded(space0, "VTABLE_"),
            ident: terminated(take_until(1.., "{"), "{ "),
            variants: repeat(len, terminated(variant_offset, (opt(","), space0))),
            _: "};",
            _: space0,
            _: opt(newline)
        }
    }
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_offset_table() {
        let input = "constexpr std::array<REL::VariantOffset, 1> VTABLE_std___Ref_count_obj_BSLocklessSimpleList_ActiveEffect_____Node_{ REL::VariantOffset(0, 0, 0x15a4358) };";
        let expected = VariantOffsetTable {
            ident: "std___Ref_count_obj_BSLocklessSimpleList_ActiveEffect_____Node_",
            len: 1,
            variants: vec![VariantOffset { se_offset: 0, ae_offset: 0, vr_offset: 0x15a4358 }],
        };

        match variant_offset_table.parse(input) {
            Ok(variant_table) => assert_eq!(variant_table, expected),
            Err(err) => panic!("Error:\n{err}"),
        }
    }
}
