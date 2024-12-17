use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CODATA, COLON, COMMA},
    util::BracesExt,
    DocAllocator, Print,
};

use crate::{
    syntax::{context::TypingContext, types::Ty, Name},
    typing::{errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct DtorSig {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: TypingContext,
    pub cont_ty: Ty,
}

impl DtorSig {
    fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        self.args.check(symbol_table)?;
        self.cont_ty.check(symbol_table)?;
        Ok(())
    }
}

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct CodataDeclaration {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub dtors: Vec<DtorSig>,
}

impl CodataDeclaration {
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        for dtor in &self.dtors {
            dtor.check(symbol_table)?;
        }
        Ok(())
    }
}
impl From<CodataDeclaration> for Declaration {
    fn from(codata: CodataDeclaration) -> Declaration {
        Declaration::CodataDeclaration(codata)
    }
}

impl Print for CodataDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(CODATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());

        let body = if self.dtors.is_empty() {
            alloc.space().braces_anno()
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.dtors.iter().map(|dtor| dtor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
                .braces_anno()
        };

        head.append(body.group())
    }
}

impl Print for DtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .dtor(&self.name)
            .append(self.args.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.cont_ty.print(cfg, alloc))
    }
}

#[cfg(test)]
mod codata_declaration_tests {
    use crate::{
        test_common::codata_stream,
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };
    use printer::Print;

    #[test]
    fn display_stream() {
        let result = codata_stream().print_to_string(Default::default());
        let expected = "codata StreamInt { Hd: i64, Tl: StreamInt }";
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_check() {
        let mut symbol_table = SymbolTable::default();
        codata_stream().build(&mut symbol_table).unwrap();
        let result = codata_stream().check(&symbol_table);
        assert!(result.is_ok())
    }
}
