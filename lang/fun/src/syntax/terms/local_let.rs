use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, EQ, IN, LET},
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        types::{OptTyped, Ty},
        XVar,
    },
    traits::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Let {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub variable: XVar,
    pub var_ty: Ty,
    pub bound_term: Rc<Term>,
    pub in_term: Rc<Term>,
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
            .append(alloc.line())
            .append(alloc.keyword(IN))
            .append(alloc.space())
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
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let bound_checked = self.bound_term.check(symbol_table, context, &self.var_ty)?;
        let mut new_context = context.clone();
        new_context.bindings.push(ContextBinding::TypedVar {
            var: self.variable.clone(),
            ty: self.var_ty.clone(),
        });
        let in_checked = self.in_term.check(symbol_table, &new_context, expected)?;
        Ok(Let {
            bound_term: bound_checked,
            in_term: in_checked,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

impl UsedBinders for Let {
    fn used_binders(&self, used: &mut HashSet<XVar>) {
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
            context::TypingContext,
            substitution::SubstitutionBinding,
            terms::{Constructor, Let, Lit, Var},
            types::Ty,
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
            in_term: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
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
                Var {
                    span: Span::default(),
                    ty: Some(Ty::mk_i64()),
                    var: "x".to_owned(),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_let_fail() {
        let symbol_table = symbol_table_list();
        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_i64(),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![SubstitutionBinding::TermBinding(Var::mk("x").into())],
                    ty: None,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext::default(),
            &Ty::mk_decl("ListInt"),
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
            "let x : i64 = 2\nin 4"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x : i64 = 2 in 4"), Ok(example().into()));
    }
}
