//! This module contains the definition of top-level functions.

use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COLON, COMMA, DEF},
    util::BracesExt,
};

use crate::{
    syntax::{context::TypingContext, names::Name, terms::Term, types::Ty},
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::Declaration;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, a return type, and the body
/// term.
///
/// Example:
/// ```text
/// def fac(n: i64): i64 { if n == 0 { 1 } else { n * fac(n - 1) } }
/// ```
/// The top-level function named `fac` has a single (producer) parameter of type `i64` and returns
/// an `i64`. Its body is contained within `{...}`
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Def {
    /// The Source Location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The name of the definition
    pub name: Name,
    /// The parameters
    pub context: TypingContext,
    /// The return type
    pub ret_ty: Ty,
    /// The body term
    pub body: Term,
}

impl Def {
    /// This function checks the well-formedness of the top-level function. This consists of
    /// checking the well-formedness of the paramater list and return type, and typechecking the
    /// body in the context given by the parameters.
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
        let sep = alloc.text(COMMA).append(alloc.line());
        let params = alloc
            .line_()
            .append(
                alloc.intersperse(
                    self.context
                        .bindings
                        .iter()
                        .map(|binding| binding.print(cfg, alloc)),
                    sep,
                ),
            )
            .nest(cfg.indent)
            .append(alloc.line_())
            .parens();

        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
            .append(params.group())
            .append(COLON)
            .append(alloc.space())
            .append(self.ret_ty.print(cfg, alloc))
            .append(alloc.space());

        let body = alloc
            .hardline()
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.hardline())
            .braces_anno();

        head.append(body)
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
        parser::fun,
        syntax::{
            context::TypingContext,
            program::Program,
            terms::{Lit, Term},
            types::Ty,
        },
        test_common::{data_list, def_mult, def_mult_typed},
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    use super::Def;

    /// A definition with no arguments.
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
        let parser = fun::ProgParser::new();
        let module = Program {
            declarations: vec![simple_def().into()],
        };
        assert_eq!(parser.parse("def x() : i64 { 4 }"), Ok(module));
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
