use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::DOT, DocAllocator, Print};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        substitution::Substitution,
        types::{OptTyped, Ty, TypeArgs},
        used_binders::UsedBinders,
        Name, Var,
    },
    typing::{
        check::{check_args, check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use std::{collections::HashSet, rc::Rc};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub id: Name,
    pub destructee: Rc<Term>,
    pub type_args: TypeArgs,
    pub args: Substitution,
    pub ty: Option<Ty>,
}

impl OptTyped for Destructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Destructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let print_args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        self.destructee
            .print(cfg, alloc)
            .append(DOT)
            .append(alloc.dtor(&self.id))
            .append(self.type_args.print(cfg, alloc))
            .append(print_args)
    }
}

impl From<Destructor> for Term {
    fn from(value: Destructor) -> Self {
        Term::Destructor(value)
    }
}

impl Check for Destructor {
    fn check(
        self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let dtor_name = self.id.clone() + &self.type_args.print_to_string(None);
        let ty = match symbol_table.lookup_ty_for_dtor(&self.span.to_miette(), &dtor_name) {
            Ok(ty) => ty,
            Err(_) => symbol_table.lookup_ty_template_for_dtor(&self.id, &self.type_args)?,
        };
        let destructee_checked = self.destructee.check(symbol_table, context, &ty)?;
        match symbol_table.dtors.get(&dtor_name) {
            Some(signature) => {
                let (types, ret_ty) = signature.clone();
                check_equality(&self.span, symbol_table, expected, &ret_ty)?;
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    &types,
                )?;
                Ok(Destructor {
                    destructee: destructee_checked,
                    args: new_args,
                    ty: Some(expected.clone()),
                    ..self
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}

impl UsedBinders for Destructor {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.destructee.used_binders(used);
        self.args.used_binders(used);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::Check;
    use crate::{
        parser::fun,
        syntax::{
            context::{
                Chirality::{Cns, Prd},
                TypingContext,
            },
            terms::{Destructor, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::{symbol_table_fun_template, symbol_table_lpair},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_fst() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        let mut symbol_table = symbol_table_lpair();
        let result = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![],
            destructee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            args: vec![],
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            destructee: Rc::new(
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl(
                        "LPair",
                        TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                    )),
                    chi: Some(Prd),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ap() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        ctx.add_covar("a", Ty::mk_i64());
        let mut symbol_table = symbol_table_fun_template();
        let result = Destructor {
            span: Span::default(),
            id: "Ap".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![Lit::mk(1).into(), XVar::mk("a").into()],
            destructee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "Ap".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![
                Lit::mk(1).into(),
                XVar {
                    span: Span::default(),
                    var: "a".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Cns),
                }
                .into(),
            ],
            destructee: Rc::new(
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl(
                        "Fun",
                        TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                    )),
                    chi: Some(Prd),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_dtor_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_i64()])));
        let result = Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![],
            destructee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    /// "x.hd"
    fn example_1() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            destructee: Rc::new(XVar::mk("x").into()),
            args: vec![],
            ty: None,
        }
    }

    /// "x.hd.hd"
    fn example_2() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            destructee: Rc::new(example_1().into()),
            args: vec![],
            ty: None,
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(example_1().print_to_string(Default::default()), "x.Hd[i64]")
    }

    #[test]
    fn display_2() {
        assert_eq!(
            example_2().print_to_string(Default::default()),
            "x.Hd[i64].Hd[i64]"
        )
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            destructee: Rc::new(XVar::mk("x").into()),
            args: vec![XVar::mk("y").into(), XVar::mk("z").into()],
            ty: None,
        };
        let result = dest.print_to_string(Default::default());
        let expected = "x.Fst[i64, i64](y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.Hd[i64]"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.Hd[i64].Hd[i64]"), Ok(example_2().into()));
    }
}
