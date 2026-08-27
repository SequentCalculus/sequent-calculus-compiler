//! This module defines a pattern match of a data type in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{CASE, DOT};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::collections::HashMap;
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
    pub span: SourceSpan,
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
                match symbol_table.lookup_ty_for_ctor(&self.span, &ctor_name) {
                    Ok(ty) => ty,
                    Err(_) => {
                        // if there is no instance yet, we create on from the template
                        symbol_table.lookup_ty_template_for_ctor(&clause.xtor, &self.type_args)?
                    }
                }
            }
            None => {
                return Err(Error::EmptyMatch { span: self.span });
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
                    span: self.span,
                    ctor,
                });
            };

            let (own_type_params, signature) = match symbol_table.ctors.get(&ctor_name) {
                None => {
                    return Err(Error::Undefined {
                        span: Some(self.span),
                        name: ctor_name.clone(),
                    });
                }
                Some((own_type_params, signature)) => (own_type_params.clone(), signature.clone()),
            };

            clause.context_names.no_dups(&ctor_name)?;

            // The constructor's own type parameters are existentially bound by this clause,
            // e.g. `B` in `Cons[B](x, xs)`. The user-chosen name is used directly as a rigid,
            // opaque type variable, valid only within this clause's body.
            if clause.type_params.bindings.len() != own_type_params.bindings.len() {
                return Err(Error::WrongNumberOfTypeArguments {
                    span: Some(clause.span),
                    expected: own_type_params.bindings.len(),
                    got: clause.type_params.bindings.len(),
                });
            }
            let rigid_args = symbol_table.push_abstract_vars(
                &clause.span,
                &clause.type_params,
                &own_type_params,
            )?;

            let mappings: HashMap<Name, Ty> = own_type_params
                .names()
                .into_iter()
                .zip(rigid_args)
                .collect();
            let signature = signature.subst_ty(&mappings);
            let context_clause = clause.context_names.add_types(&signature)?;

            let mut new_context = context.clone();
            new_context
                .bindings
                .append(&mut context_clause.bindings.clone());

            clause.context = context_clause;
            clause.body = clause.body.check(symbol_table, &new_context, expected)?;

            symbol_table.pop_abstract_vars(&clause.type_params);

            new_clauses.push(clause);
        }

        if !self.clauses.is_empty() {
            return Err(Error::UnexpectedCtorsInCase {
                span: self.span,
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
    use printer::*;

    use crate::parser::fun;
    use crate::syntax::context::ContextBinding;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::*;

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
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    type_params: TypeContext::default(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    type_params: TypeContext::default(),
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
            span: dummy_span(),
            clauses: vec![
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Nil".to_owned(),
                    type_params: TypeContext::default(),
                    context_names: NameContext::default(),
                    context: TypingContext::default(),
                    body: Lit::mk(1).into(),
                },
                Clause {
                    span: dummy_span(),
                    pol: Polarity::Data,
                    xtor: "Cons".to_owned(),
                    type_params: TypeContext::default(),
                    context_names: ctx_case_names,
                    context: ctx_case,
                    body: XVar {
                        span: dummy_span(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_i64()),
                        chi: Some(Prd),
                    }
                    .into(),
                },
            ],
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
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
            span: dummy_span(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                type_params: TypeContext::default(),
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
            span: dummy_span(),
            scrutinee: Rc::new(XVar::mk("x").into()),
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
            span: dummy_span(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Tup".to_owned(),
                type_params: TypeContext::default(),
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

    /// Builds a symbol table containing a template `data Any { Mk[B](x: B) }`, i.e. a data type
    /// without its own type parameters whose single constructor has its own existential type
    /// parameter `B`.
    fn symbol_table_any_existential_template() -> SymbolTable {
        let mut symbol_table = SymbolTable::default();

        symbol_table.type_templates.insert(
            "Any".to_owned(),
            (
                Polarity::Data,
                TypeParams::default(),
                vec!["Mk".to_owned()],
            ),
        );

        symbol_table.ctor_templates.insert(
            "Mk".to_owned(),
            (
                TypeParams::mk(&[("B", Polarity::Data)]),
                TypingContext {
                    span: None,
                    bindings: vec![ContextBinding {
                        var: "x".to_owned(),
                        chi: Prd,
                        ty: Ty::mk_decl("B", TypeArgs::default()),
                    }],
                },
            ),
        );

        symbol_table
    }

    #[test]
    fn check_existential_ctor_in_case() {
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());

        let mut ctx = TypingContext::default();
        ctx.add_var("y", Ty::mk_decl("Any", TypeArgs::default()));

        let mut symbol_table = symbol_table_any_existential_template();
        let result = Case {
            span: dummy_span(),
            scrutinee: Rc::new(XVar::mk("y").into()),
            type_args: TypeArgs::default(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Mk".to_owned(),
                // "B" is bound by the clause itself, as in `Mk[B](x) => ...`
                type_params: TypeContext {
                    span: None,
                    bindings: vec!["B".to_owned()],
                },
                context_names: ctx_names.clone(),
                context: TypingContext::default(),
                body: Lit::mk(1).into(),
            }],
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();

        let mut expected_clause_ctx = TypingContext::default();
        expected_clause_ctx.add_var("x", Ty::mk_decl("B", TypeArgs::default()));

        let expected = Case {
            span: dummy_span(),
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
                    var: "y".to_owned(),
                    ty: Some(Ty::mk_decl("Any", TypeArgs::default())),
                    chi: Some(Prd),
                }
                .into(),
            ),
            type_args: TypeArgs::default(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Mk".to_owned(),
                type_params: TypeContext {
                    span: None,
                    bindings: vec!["B".to_owned()],
                },
                context_names: ctx_names,
                context: expected_clause_ctx,
                body: Lit::mk(1).into(),
            }],
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_existential_ctor_wrong_arity_in_case() {
        // "Mk(x) => 1" is missing the required binder for the constructor's own type parameter `B`.
        let mut ctx_names = NameContext::default();
        ctx_names.bindings.push("x".to_string());

        let mut ctx = TypingContext::default();
        ctx.add_var("y", Ty::mk_decl("Any", TypeArgs::default()));

        let mut symbol_table = symbol_table_any_existential_template();
        let result = Case {
            span: dummy_span(),
            scrutinee: Rc::new(XVar::mk("y").into()),
            type_args: TypeArgs::default(),
            clauses: vec![Clause {
                span: dummy_span(),
                pol: Polarity::Data,
                xtor: "Mk".to_owned(),
                type_params: TypeContext::default(),
                context_names: ctx_names,
                context: TypingContext::default(),
                body: Lit::mk(1).into(),
            }],
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }
}
