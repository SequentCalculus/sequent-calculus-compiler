use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COLON, DEF},
    util::BracesExt,
};

use crate::{
    syntax::{Name, context::TypingContext, terms::Term, types::Ty},
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

/// A top-level function definition in a module.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Def {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub context: TypingContext,
    pub body: Term,
    pub ret_ty: Ty,
}

impl Def {
    pub fn check(mut self, symbol_table: &mut SymbolTable) -> Result<Def, Error> {
        self.context.no_dups(&self.name)?;
        self.context.check(symbol_table)?;
        self.ret_ty.check(&self.span, symbol_table)?;

        self.body = self.body.check(symbol_table, &self.context, &self.ret_ty)?;

        Ok(self)
    }
}

impl Print for Def {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.clone())
            .append(self.context.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.ret_ty.print(cfg, alloc))
            .append(alloc.space());

        let body = alloc
            .line()
            .append(self.body.print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno();

        head.append(body).group()
    }
}

impl From<Def> for Declaration {
    fn from(value: Def) -> Self {
        Declaration::Def(value)
    }
}

#[cfg(test)]
mod def_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{
        parser::parse_module,
        syntax::{
            context::TypingContext,
            declarations::Module,
            terms::{Lit, Term},
            types::Ty,
        },
        test_common::{data_list, def_mult, def_mult_typed},
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    use super::Def;

    /// A definition with no arguments:
    fn simple_def() -> Def {
        Def {
            span: Span::default(),
            name: "x".to_string(),
            context: TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_i64(),
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            simple_def().print_to_string(Default::default()),
            "def x: i64 { 4 }".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let module = Module {
            declarations: vec![simple_def().into()],
        };
        assert_eq!(parse_module("def x() : i64 { 4 }"), Ok(module));
    }

    #[test]
    fn def_check() {
        let mut symbol_table = SymbolTable::default();
        def_mult().build(&mut symbol_table).unwrap();
        data_list().build(&mut symbol_table).unwrap();
        let result = def_mult().check(&mut symbol_table).unwrap();
        let expected = def_mult_typed();
        assert_eq!(result, expected)
    }
}
