use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, COLONEQ, DEF, SEMI},
    DocAllocator, Print,
};

use crate::{
    syntax::{context::TypingContext, terms::Term, types::Ty, Name},
    typing::{
        check::{check_type, context::check_typing_context, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
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
        check_typing_context(&self.context, symbol_table)?;
        check_type(&self.ret_ty, symbol_table)?;
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
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.ret_ty.print(cfg, alloc))
            .append(alloc.space())
            .append(COLONEQ);

        let body = alloc
            .line()
            .append(self.body.print(cfg, alloc))
            .append(SEMI)
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
            declarations::Module,
            terms::{Lit, Term},
            types::Ty,
        },
    };

    use super::Definition;

    /// A definition with no arguments:
    fn simple_definition() -> Definition {
        Definition {
            span: Span::default(),
            name: "x".to_string(),
            context: vec![],
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_int(),
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            simple_definition().print_to_string(Default::default()),
            "def x() : Int := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        let module = Module {
            declarations: vec![simple_definition().into()],
        };
        assert_eq!(parser.parse("def x() : Int := 4;"), Ok(module));
    }
}
