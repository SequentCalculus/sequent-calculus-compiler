//! This module defines let-bindings of a term in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::{COLON, EQ, LET, SEMI};
use printer::*;
use std::collections::HashMap;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

/// This struct defines let-bindings of a term. It consists of the variable the term is bound to,
/// the annotated type of the bound term, the bound term, the remaining term in which the binding
/// is visible, and after typechecking also of the inferred type of the entire term.
///
/// Example:
/// ```text
/// let x: i64 = 2 * 2; x
/// ```
/// This binds the variable `x` of type `i64` to `2 * 2` and then returns it.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Let {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The bound variable
    pub variable: Var,
    /// The (annotated) type of the bound term
    pub var_ty: Ty,
    /// The bound term
    pub bound_term: Rc<Term>,
    /// The term in which the variable for the bound term is in scope
    pub in_term: Rc<Term>,
    /// The (inferred) type of the entire term
    pub ty: Option<Ty>,
}

impl SubstType for Let {
    fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> Self {
        self.var_ty = self.var_ty.subst_ty(mappings);
        self.bound_term = self.bound_term.subst_ty(mappings);
        self.in_term = self.in_term.subst_ty(mappings);
        self.ty = self.ty.subst_ty(mappings);
        self
    }
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
            .append(self.variable.print(cfg, alloc))
            .append(COLON)
            .append(alloc.space())
            .append(self.var_ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(self.bound_term.print(cfg, alloc).group())
            .append(SEMI)
            .append(alloc.hardline())
            .append(self.in_term.print(cfg, alloc).group())
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
    use codespan::Span;
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::*;

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
                    args: vec![XVar::mk("x").into()].into(),
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
            "let x: i64 = 2;\n4"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x: i64 = 2; 4"), Ok(example().into()));
    }
}
