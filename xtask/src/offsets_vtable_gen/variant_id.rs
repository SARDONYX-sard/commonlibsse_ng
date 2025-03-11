use winnow::{
    ModalResult, Parser,
    ascii::{digit1, newline, space0},
    combinator::{opt, preceded, repeat, seq, terminated},
    token::take_until,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariantID {
    pub se_id: u32,
    pub ae_id: u32,
    pub vr_offset: u64,
}

impl core::fmt::Display for VariantID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { se_id, ae_id, vr_offset } = *self;
        write!(f, "VariantID::new({se_id}, {ae_id}, {vr_offset:#x})")
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariantIDTable<'a> {
    pub ident: &'a str,
    pub len: usize,
    pub variants: Vec<VariantID>,
}

impl core::fmt::Display for VariantIDTable<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { ident, len, variants } = self;
        write!(
            f,
            "pub const VTABLE_{ident}: [VariantID; {len}] = [{}];",
            variants.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")
        )
    }
}

fn variant_id(input: &mut &str) -> ModalResult<VariantID> {
    seq! {
        VariantID {
            _: "REL::VariantID(",
            se_id: terminated(digit1, ", ").parse_to(),
            ae_id: terminated(digit1, ", ").parse_to(),
            vr_offset: crate::offsets_rtti_gen::hex,
            _: ")",
        }
    }
    .parse_next(input)
}

pub(crate) fn variant_id_table<'a>(input: &mut &'a str) -> ModalResult<VariantIDTable<'a>> {
    seq! {
        VariantIDTable {
            _: space0,
            _: "constexpr std::array<REL::VariantID, ",
            len: terminated(digit1.parse_to::<usize>(), ">"),
            _: preceded(space0, "VTABLE_"),
            ident: terminated(take_until(1.., "{"), "{ "),
            variants: repeat(len, terminated(variant_id, (opt(","), space0))),
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
    fn test_one_line() {
        let input = r#"constexpr std::array<REL::VariantID, 1>  VTABLE_ConcreteFormFactory_AlchemyItem_46_{ REL::VariantID(228356, 186192, 0x1596bc0) };"#;
        let expected = VariantIDTable {
            ident: "ConcreteFormFactory_AlchemyItem_46_",
            len: 1,
            variants: vec![VariantID { se_id: 228356, ae_id: 186192, vr_offset: 0x1596bc0 }],
        };

        match variant_id_table.parse(input) {
            Ok(variant_table) => assert_eq!(variant_table, expected),
            Err(err) => panic!("Error:\n{err}"),
        }
    }
}
