use printer::{
    tokens::{COMMA, MINUS, PLUS, TIMES},
    util::BracesExt,
    Alloc, Builder, DocAllocator, Print, PrintCfg,
};
use terms::Clause;

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

pub mod context;
pub mod declarations;
pub mod substitution;
pub mod terms;
pub mod types;

// Prints "{ }"
pub(crate) fn empty_braces<'a>(alloc: &'a Alloc<'a>) -> Builder<'a> {
    alloc.space().braces_anno()
}

pub fn print_cases<'a>(
    cases: &'a [Clause<Name>],
    cfg: &PrintCfg,
    alloc: &'a Alloc<'a>,
) -> Builder<'a> {
    match cases.len() {
        0 => empty_braces(alloc),

        1 => alloc
            .line()
            .append(cases[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

// BinOp
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Prod,
    Sum,
    Sub,
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}
