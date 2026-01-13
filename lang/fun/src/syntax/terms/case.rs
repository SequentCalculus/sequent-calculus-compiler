//! This module defines a pattern match of a data type in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::{CASE, DOT};
use printer::*;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

/// This struct defines a pattern match of a data type. It consists of the scrutinee on which to
/// match, a list of type arguments instantiating the type parameters of the data type, a list of
/// clauses, and after typechecking also of the inferred type.
///
/// Example:
/// ```text
/// l.case[i64] { Nil => 0, Cons(x, xs) => 1 + len(xs) }
/// ```
/// matches on list `l` with type argument `i64`, i.e., requires the list to be `List[i64]`. It
/// has clauses for the patterns `Nil` and `Cons(x, xs)`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The term to be matched on
    pub scrutinee: Rc<Term>,
    /// The type arguments instantiating the type parameters of the type
    pub type_args: TypeArgs,
    /// The list of clauses
    pub clauses: Vec<Clause>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Case {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Case {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if matches!(*self.scrutinee, Term::Destructor(_)) {
            self.scrutinee
                .print(cfg, alloc)
                .append(alloc.line_())
                .append(DOT)
                .append(alloc.keyword(CASE))
                .append(self.type_args.print(cfg, alloc))
                .append(alloc.space())
                .append(print_clauses(&self.clauses, cfg, alloc))
                .nest(cfg.indent)
                .align()
        } else {
            self.scrutinee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.keyword(CASE))
                .append(self.type_args.print(cfg, alloc))
                .append(alloc.space())
                .append(print_clauses(&self.clauses, cfg, alloc))
        }
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
        // Find out the type on which we pattern match by inspecting the first clause. We throw an
        // error for empty cases.
        let (ty, expected_ctors) = match self.clauses.first() {
            Some(clause) => {
                // the name of the constructor in the symbol table for the instantiated data type
                let ctor_name = clause.xtor.clone() + &self.type_args.print_to_string(None);
                match symbol_table.lookup_ty_for_ctor(&self.span.to_miette(), &ctor_name) {
                    Ok(ty) => ty,
                    Err(_) => {
                        // if there is no instance yet, we create on from the template
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

        // We check the scrutinee `e` in `e.case {...}` against this type.
        self.scrutinee = self.scrutinee.check(symbol_table, context, &ty)?;

        let mut new_clauses = vec![];
        for ctor in expected_ctors {
            // the name of the constructor in the symbol table for the instantiated data type
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
        self.scrutinee.used_binders(used);
        self.clauses.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use codespan::Span;
    use printer::*;

    use crate::parser::fun;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::*;

    use std::rc::Rc;

    #[test]
    fn check_case_list() {
        let mut ctx_case_names = VarContext::default();
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
                    context_names: VarContext::default(),
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
            scrutinee: Rc::new(XVar::mk("x").into()),
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
                    context_names: VarContext::default(),
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
            scrutinee: Rc::new(
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
        let mut ctx_names = VarContext::default();
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
            scrutinee: Rc::new(Lit::mk(1).into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            ty: None,
        }
        .check(&mut symbol_table, &TypingContext::default(), &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example_empty() -> Case {
        Case {
            span: Span::default(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::default(),
            clauses: vec![],
            ty: None,
        }
    }

    fn example_tup() -> Case {
        let mut ctx_names = VarContext::default();
        ctx_names.bindings.push("x".to_string());
        ctx_names.bindings.push("y".to_string());
        Case {
            span: Span::default(),
            scrutinee: Rc::new(XVar::mk("x").into()),
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
