use printer::{theme::ThemeExt, Print};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Chirality {
    Prd,
    Cns,
    Ext,
}

impl Print for Chirality {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Chirality::Prd => alloc.keyword("prd"),
            Chirality::Cns => alloc.keyword("cns"),
            Chirality::Ext => alloc.keyword("ext"),
        }
    }
}

#[cfg(test)]
mod chirality_tests {
    use super::Chirality;
    use printer::Print;

    #[test]
    fn print_prd() {
        let result = Chirality::Prd.print_to_string(Default::default());
        let expected = "prd";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_cns() {
        let result = Chirality::Cns.print_to_string(Default::default());
        let expected = "cns";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_ext() {
        let result = Chirality::Ext.print_to_string(Default::default());
        let expected = "ext";
        assert_eq!(result, expected)
    }
}
