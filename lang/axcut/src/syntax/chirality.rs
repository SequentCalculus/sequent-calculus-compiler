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
