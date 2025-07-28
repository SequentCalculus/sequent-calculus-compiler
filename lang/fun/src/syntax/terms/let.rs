//! Defines [Let]
use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COLON, EQ, LET, SEMI},
};

use super::Term;
use crate::{
    syntax::{
        Var,
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

/// A let-binding
/// Example: `let x : i64 = 2 * 2; x
/// Binds the variable `x` of type `i64` to `2*2` and returns it
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Let {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The bound variable
    pub variable: Var,
    /// The type of the bound term (annotated)
    pub var_ty: Ty,
    /// The bound term
    pub bound_term: Rc<Term>,
    /// The term in which the variable is bound
    pub in_term: Rc<Term>,
    /// The type of the entire term (inferred)
    pub ty: Option<Ty>,
}

impl OptTyped for Let {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Let {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LET)
            .append(alloc.space())
            .append(self.variable.clone())
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.var_ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(self.bound_term.print(cfg, alloc))
            .append(SEMI)
            .append(alloc.line())
            .append(self.in_term.print(cfg, alloc))
            .align()
    }
}

impl From<Let> for Term {
    fn from(value: Let) -> Self {
        Term::Let(value)
    }
}

impl Check for Let {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        self.var_ty.check(&self.span, symbol_table)?;
        self.bound_term = self.bound_term.check(symbol_table, context, &self.var_ty)?;

        let mut new_context = context.clone();
        new_context.add_var(&self.variable, self.var_ty.clone());
        self.in_term = self.in_term.check(symbol_table, &new_context, expected)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Let {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.variable.clone());
        self.bound_term.used_binders(used);
        self.in_term.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::{
            context::{Chirality::Prd, TypingContext},
            terms::{Constructor, Let, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::symbol_table_list,
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_let1() {
        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_i64(),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_i64(),
        )
        .unwrap();
        let expected = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_i64(),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(
                XVar {
                    span: Span::default(),
                    ty: Some(Ty::mk_i64()),
                    var: "x".to_owned(),
                    chi: Some(Prd),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_let_fail() {
        let mut symbol_table = symbol_table_list();
        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_i64(),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![XVar::mk("x").into()],
                    ty: None,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &mut symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err())
    }

    fn example() -> Let {
        Let {
            span: Span::default(),
            variable: "x".to_string(),
            var_ty: Ty::mk_i64(),
            bound_term: Rc::new(Term::Lit(Lit::mk(2))),
            in_term: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "let x : i64 = 2;\n4"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x : i64 = 2; 4"), Ok(example().into()));
    }
}
