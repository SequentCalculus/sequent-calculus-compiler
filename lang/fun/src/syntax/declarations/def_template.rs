//! This module contains the definition of top-level functions.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::{COLON, DEF};
use printer::*;
use std::collections::HashMap;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::typing::*;

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
pub struct DefTemplate {
    /// The Source Location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The name of the definition
    pub name: Name,
    /// The type paramenters
    pub type_params: TypeContext,
    /// The parameters
    pub context: TypingContext,
    /// The return type
    pub ret_ty: Ty,
    /// The body term
    pub body: Term,
}

impl DefTemplate {
    /// This function checks the well-formedness of the top-level template. This consists of
    /// checking the well-formedness of the paramater list and return type
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        self.type_params.no_dups(&self.name)?;
        self.context
            .check_template(symbol_table, &self.type_params)?;
        self.ret_ty
            .check_template(&self.span, symbol_table, &self.type_params)?;
        Ok(())
    }

    pub fn instantiate(&self, span: Span, args: Vec<Ty>) -> Result<Def, Error> {
        if self.type_params.bindings.len() != args.len() {
            return Err(Error::WrongNumberOfTypeArguments {
                span: span.to_miette(),
                expected: self.type_params.bindings.len(),
                got: args.len(),
            });
        }
        let mappings = self
            .type_params
            .bindings
            .iter()
            .cloned()
            .zip(args.into_iter())
            .collect();
        let new_context = self.context.clone().subst_ty(&mappings);
        let new_ret = self.ret_ty.clone().subst_ty(&mappings);
        let new_body = self.body.clone().subst_ty(&mappings);
        Ok(Def {
            span: self.span,
            name: self.name.clone(),
            context: new_context,
            ret_ty: new_ret,
            body: new_body,
        })
    }
}

impl Print for DefTemplate {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
            .append(self.type_params.print(cfg, alloc))
            .append(self.context.print(cfg, alloc).parens())
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

        head.group().append(body)
    }
}

impl From<DefTemplate> for Declaration {
    fn from(value: DefTemplate) -> Self {
        Declaration::DefTemplate(value)
    }
}

#[cfg(test)]
mod def_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::{TypeContext, TypingContext},
            program::Program,
            terms::{Lit, Term},
            types::Ty,
        },
        test_common::{data_list, def_mult, def_mult_typed},
        typing::symbol_table::{BuildSymbolTable, SymbolTable},
    };

    use super::DefTemplate;

    /// A definition with no arguments.
    fn simple_def() -> DefTemplate {
        DefTemplate {
            span: Span::default(),
            name: "x".to_string(),
            type_params: TypeContext {
                span: Span::default(),
                bindings: Vec::new(),
            },
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
            "def x(): i64 {\n    4\n}".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        let module = Program {
            declarations: vec![simple_def().into()],
        };
        assert_eq!(parser.parse("def x(): i64 { 4 }"), Ok(module));
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
