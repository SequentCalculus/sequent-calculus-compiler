use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, EQ, IN, LET},
    DocAllocator, Print,
};

use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        types::{OptTyped, Ty},
        Variable,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Let {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub variable: Variable,
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
        new_context.push(ContextBinding::TypedVar {
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

#[cfg(test)]
mod test {
    use super::Check;
    use super::Term;
    use crate::parser::fun;
    use crate::{
        syntax::{
            context::ContextBinding,
            substitution::SubstitutionBinding,
            terms::{Constructor, Let, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_let1() {
        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(
                Var {
                    span: Span::default(),
                    ty: Some(Ty::mk_int()),
                    var: "x".to_owned(),
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_let_fail() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table.ctors.insert("Nil".to_owned(), vec![]);
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
        );
        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_int(),
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
        .check(&symbol_table, &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }

    fn example() -> Let {
        Let {
            span: Span::default(),
            variable: "x".to_string(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(Term::Lit(Lit::mk(2))),
            in_term: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "let x : Int = 2\nin 4"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x : Int = 2 in 4"), Ok(example().into()));
    }
}
