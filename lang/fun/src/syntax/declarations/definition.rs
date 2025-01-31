use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, DEF},
    DocAllocator, Print,
};

use crate::{
    syntax::{context::TypingContext, terms::Term, types::Ty, Name},
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

/// A toplevel function definition in a module.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Definition {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub context: TypingContext,
    pub body: Term,
    pub ret_ty: Ty,
}

impl Definition {
    pub fn check(self, symbol_table: &SymbolTable) -> Result<Definition, Error> {
        self.context.check(symbol_table)?;
        self.context.no_dups(self.name.clone())?;
        self.ret_ty.check(symbol_table)?;
        let body_checked = self.body.check(symbol_table, &self.context, &self.ret_ty)?;
        Ok(Definition {
            body: body_checked,
            ..self
        })
    }
}

impl Print for Definition {
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
            .append(self.ret_ty.print(cfg, alloc));

        let body = alloc
            .line()
            .append(
                alloc
                    .space()
                    .append(self.body.print(cfg, alloc))
                    .append(alloc.space())
                    .braces(),
            )
            .nest(cfg.indent);

        head.append(body).group()
    }
}

impl From<Definition> for Declaration {
    fn from(value: Definition) -> Self {
        Declaration::Definition(value)
    }
}

#[cfg(test)]
mod definition_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            declarations::Module,
            terms::{Lit, Term},
            types::Ty,
        },
        test_common::{data_list, def_mult, def_mult_typed},
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    use super::Definition;

    /// A definition with no arguments:
    fn simple_definition() -> Definition {
        Definition {
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
            simple_definition().print_to_string(Default::default()),
            "def x: i64 { 4 }".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        let module = Module {
            declarations: vec![simple_definition().into()],
        };
        assert_eq!(parser.parse("def x() : i64 { 4 }"), Ok(module));
    }

    #[test]
    fn def_check() {
        let mut symbol_table = SymbolTable::default();
        def_mult().build(&mut symbol_table).unwrap();
        data_list().build(&mut symbol_table).unwrap();
        let result = def_mult().check(&symbol_table).unwrap();
        let expected = def_mult_typed();
        assert_eq!(result, expected)
    }
}
