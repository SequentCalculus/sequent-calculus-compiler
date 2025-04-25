use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COMMA, DATA},
    util::BracesExt,
};

use crate::{
    syntax::{
        Name,
        context::{TypeContext, TypingContext},
    },
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
    fn check(&self, symbol_table: &SymbolTable, type_params: &TypeContext) -> Result<(), Error> {
        self.args.check_template(symbol_table, type_params)?;
        Ok(())
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

#[derive(Derivative, Clone, Debug)]
#[derivative(PartialEq, Eq)]
pub struct Data {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub type_params: TypeContext,
    pub ctors: Vec<CtorSig>,
}

impl Data {
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        for ctor in &self.ctors {
            ctor.check(symbol_table, &self.type_params)?;
        }
        Ok(())
    }
}

impl From<Data> for Declaration {
    fn from(data: Data) -> Declaration {
        Declaration::Data(data)
    }
}

impl Print for Data {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DATA)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(self.type_params.print(cfg, alloc))
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

#[cfg(test)]
mod data_tests {
    use printer::Print;

    use crate::{
        test_common::data_list,
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    #[test]
    fn display_list() {
        let result = data_list().print_to_string(Default::default());
        let expected = "data List[A] { Nil, Cons(x : A, xs : List[A]) }";
        assert_eq!(result, expected)
    }

    #[test]
    fn data_check() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let result = data_list().check(&mut symbol_table);
        assert!(result.is_ok())
    }
}
