use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, DATA},
    util::BracesExt,
    DocAllocator, Print,
};

use crate::{
    syntax::{context::TypingContext, empty_braces, Name},
    typing::{check::context::check_typing_context, errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct CtorSig {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: TypingContext,
}

impl CtorSig {
    fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        check_typing_context(&self.args, symbol_table)?;
        Ok(())
    }
}

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct DataDeclaration {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub ctors: Vec<CtorSig>,
}

impl DataDeclaration {
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        for ctor in &self.ctors {
            ctor.check(symbol_table)?;
        }
        Ok(())
    }
}

impl From<DataDeclaration> for Declaration {
    fn from(data: DataDeclaration) -> Declaration {
        Declaration::DataDeclaration(data)
    }
}

impl Print for DataDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space());

        let sep = alloc.text(COMMA).append(alloc.line());

        let body = if self.ctors.is_empty() {
            empty_braces(alloc)
        } else {
            alloc
                .line()
                .append(
                    alloc.intersperse(self.ctors.iter().map(|ctor| ctor.print(cfg, alloc)), sep),
                )
                .nest(cfg.indent)
                .append(alloc.line())
                .braces_anno()
        };

        head.append(body.group())
    }
}

impl Print for CtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .ctor(&self.name)
            .append(self.args.print(cfg, alloc).parens())
    }
}

#[cfg(test)]
mod data_declaration_tests {
    use codespan::Span;
    use printer::Print;

    use crate::syntax::{context::ContextBinding, types::Ty};

    use super::{CtorSig, DataDeclaration};

    /// Lists containing Int
    fn example_list() -> DataDeclaration {
        let nil = CtorSig {
            span: Span::default(),
            name: "Nil".to_owned(),
            args: vec![],
        };
        let cons = CtorSig {
            span: Span::default(),
            name: "Cons".to_owned(),
            args: vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
        };

        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![nil, cons],
        }
    }

    #[test]
    fn display_list() {
        let result = example_list().print_to_string(Default::default());
        let expected = "data ListInt { Nil(), Cons(x : Int, xs : ListInt) }";
        assert_eq!(result, expected)
    }
}
