use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, DATA},
    util::BracesExt,
    DocAllocator, Print,
};

use crate::{
    syntax::{context::TypingContext, Name},
    typing::{errors::Error, symbol_table::SymbolTable},
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
        self.args.check(symbol_table)?;
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
            alloc.space().braces_anno()
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
        alloc.ctor(&self.name).append(self.args.print(cfg, alloc))
    }
}

#[cfg(test)]
mod data_declaration_tests {
    use printer::Print;

    use crate::{
        test_common::data_list,
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    #[test]
    fn display_list() {
        let result = data_list().print_to_string(Default::default());
        let expected = "data ListInt { Nil, Cons(x: Int, xs: ListInt) }";
        assert_eq!(result, expected)
    }

    #[test]
    fn data_check() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let result = data_list().check(&symbol_table);
        assert!(result.is_ok())
    }
}
