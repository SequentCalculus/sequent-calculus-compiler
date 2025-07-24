use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CASE, DOT},
    DocAllocator, Print,
};

use super::{print_clauses, Clause, Term};
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty, TypeArgs},
        Var,
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

/// A Term representing a case/match expression
/// Example: `l.case[i64] { Nil => 0, Cons(x,xs) => 1 + len(xs) }}`
/// matches on `l` (which is a list) with type argument `i64`
/// i.e. requires the list to be `List[i64]`
/// In the body there are the patterns `Nil` and `Cons(x,xs)`
/// with corresponding right-hand sides `0` and `1+len(xs)`
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The term to be matched on
    pub destructee: Rc<Term>,
    /// The type arguments for the type of the destructee
    pub type_args: TypeArgs,
    /// The patterns (with right-hand sides) of the expression
    pub clauses: Vec<Clause>,
    /// The type of the term (inferred)
    pub ty: Option<Ty>,
}

impl OptTyped for Case {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Case {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.destructee
            .print(cfg, alloc)
            .append(DOT)
            .append(alloc.keyword(CASE))
            .append(self.type_args.print(cfg, alloc))
            .append(alloc.space())
            .append(print_clauses(&self.clauses, cfg, alloc))
    }
}

impl From<Case> for Term {
    fn from(value: Case) -> Self {
        Term::Case(value)
    }
}

impl Check for Case {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        // Find out the type on which we pattern match by inspecting the first clause.
        // We throw an error for empty cases.
        let (ty, expected_ctors) = match self.clauses.first() {
            Some(clause) => {
                let ctor_name = clause.xtor.clone() + &self.type_args.print_to_string(None);
                match symbol_table.lookup_ty_for_ctor(&self.span.to_miette(), &ctor_name) {
                    Ok(ty) => ty,
                    Err(_) => {
                        symbol_table.lookup_ty_template_for_ctor(&clause.xtor, &self.type_args)?
                    }
                }
            }
            None => {
                return Err(Error::EmptyMatch {
                    span: self.span.to_miette(),
                });
            }
        };

        // We check the "e" in "case e of {...}" against this type.
        self.destructee = self.destructee.check(symbol_table, context, &ty)?;

        let mut new_clauses = vec![];
        for ctor in expected_ctors {
            let ctor_name = ctor.clone() + &self.type_args.print_to_string(None);
            let mut clause = if let Some(position) =
                self.clauses.iter().position(|clause| clause.xtor == ctor)
            {
                self.clauses.swap_remove(position)
            } else {
                return Err(Error::MissingCtorInCase {
                    span: self.span.to_miette(),
                    ctor,
                });
            };
            match symbol_table.ctors.get(&ctor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: self.span.to_miette(),
                        name: ctor_name.clone(),
                    });
                }
                Some(signature) => {
                    clause.context_names.no_dups(&ctor_name)?;
                    let context_clause = clause.context_names.add_types(signature)?;

                    let mut new_context = context.clone();
                    new_context
                        .bindings
                        .append(&mut context_clause.bindings.clone());

                    clause.context = context_clause;
                    clause.body = clause.body.check(symbol_table, &new_context, expected)?;
                    new_clauses.push(clause);
                }
            }
        }

        if !self.clauses.is_empty() {
            return Err(Error::UnexpectedCtorsInCase {
                span: self.span.to_miette(),
                ctors: self
                    .clauses
                    .iter()
                    .map(|clause| clause.xtor.clone())
                    .collect::<Vec<_>>()
                    .print_to_string(None),
            });
        }
        self.clauses = new_clauses;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Case {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.destructee.used_binders(used);
        self.clauses.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::context::{Chirality::Prd, NameContext, TypingContext},
        syntax::{
            declarations::Polarity,
            terms::{Case, Clause, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::symbol_table_list_template,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_case_list() {
        let mut ctx_case_names = NameContext::default();
        ctx_case_names.bindings.push("x".to_string());
        ctx_case_names.bindings.push("xs".to_string());
        let mut ctx_case = TypingContext::default();
        ctx_case.add_var("x", Ty::mk_i64());
        ctx_case.add_var("xs", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let mut symbol_table = symbol_table_list_template();
        let result = Case {
            span: Span::default(),
            clauses: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    context_names: ctx_case_names.clone(),
                    context: TypingContext::default(),
                    body: XVar::mk("x").into(),
                },
            ],
            destructee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Case {
            span: Span::default(),
            clauses: vec![
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: Span::default(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    context_names: ctx_case_names,
                    context: ctx_case,
                    body: XVar {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_i64()),
                        chi: Some(Prd),
                    }
                    .into(),
                },
            ],
            destructee: Rc::new(
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
                    chi: Some(Prd),
                }
                .into(),
            ),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_case_fail() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        let mut symbol_table = symbol_table_list_template();
        let result = Case {
            span: Span::default(),
            clauses: vec![Clause {
                span: Span::default(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                context_names: ctx_names,
                context: TypingContext::default(),
                body: XVar::mk("x").into(),
            }],
            destructee: Rc::new(Lit::mk(1).into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            ty: None,
        }
        .check(&mut symbol_table, &TypingContext::default(), &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example_empty() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::default(),
            clauses: vec![],
            ty: None,
        }
    }

    fn example_tup() -> Case {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        Case {
            span: Span::default(),
            destructee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            clauses: vec![Clause {
                span: Span::default(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                context_names: ctx_names,
                context: TypingContext::default(),
                body: Term::Lit(Lit::mk(2)),
            }],
            ty: None,
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(
            example_empty().print_to_string(Default::default()),
            "x.case { }"
        )
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.case { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(
            example_tup().print_to_string(Default::default()),
            "x.case[i64, i64] { Tup(x, y) => 2 }"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.case[i64,i64] { Tup(x,y) => 2 }"),
            Ok(example_tup().into())
        );
    }
}
