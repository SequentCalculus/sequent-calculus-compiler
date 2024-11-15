use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CODATA, COLON, COMMA},
    util::BracesExt,
    DocAllocator, Print,
};

use crate::{
    syntax::{context::TypingContext, empty_braces, types::Ty, Name},
    typing::{
        check::{check_type, context::check_typing_context},
        errors::Error,
        symbol_table::SymbolTable,
    },
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
        check_typing_context(&self.args, symbol_table)?;
        check_type(&self.cont_ty, symbol_table)?;
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
            empty_braces(alloc)
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
            .append(self.args.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.cont_ty.print(cfg, alloc))
    }
}

#[cfg(test)]
mod codata_declaration_tests {
    use codespan::Span;
    use printer::Print;

    use crate::syntax::{context::ContextBinding, types::Ty};

    use super::{CodataDeclaration, DtorSig};

    // Streams
    fn example_stream() -> CodataDeclaration {
        let hd = DtorSig {
            span: Span::default(),
            name: "hd".to_owned(),
            args: vec![],
            cont_ty: Ty::mk_int(),
        };
        let tl = DtorSig {
            span: Span::default(),
            name: "tl".to_owned(),
            args: vec![],
            cont_ty: Ty::mk_decl("IntStream"),
        };

        CodataDeclaration {
            span: Span::default(),
            name: "IntStream".to_owned(),
            dtors: vec![hd, tl],
        }
    }

    #[test]
    fn display_stream() {
        let result = example_stream().print_to_string(Default::default());
        let expected = "codata IntStream { hd() : Int, tl() : IntStream }";
        assert_eq!(result, expected)
    }

    // Functions from Int to Int
    fn example_fun() -> CodataDeclaration {
        let ap = DtorSig {
            span: Span::default(),
            name: "ap".to_owned(),
            args: vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            cont_ty: Ty::mk_int(),
        };

        CodataDeclaration {
            span: Span::default(),
            name: "Fun".to_owned(),
            dtors: vec![ap],
        }
    }

    #[test]
    fn display_fun() {
        let result = example_fun().print_to_string(Default::default());
        let expected = "codata Fun { ap(x : Int) : Int }";
        assert_eq!(result, expected)
    }
}
