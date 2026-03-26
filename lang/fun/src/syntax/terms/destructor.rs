//! This module defines invoking destructors of codata types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::DOT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::args_insert_inferred_type;
use crate::typing::inference::{Inference, args_constraint_equations};
use crate::typing::*;

use std::collections::HashMap;
use std::{collections::HashSet, rc::Rc};

/// This struct defines an invocation of a destructor of codata type. It consists of the scrutinee
/// on which to invoke the destructor, the name of the destructor, a list of type arguments
/// instantiating the type parameters of the codata type, the arguments of the destructor, and
/// after typechecking also of the inferred type.
///
/// Example:
/// `stream.Head[i64]` invokes the destructor `Head` on a `stream` with type argument `i64`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The term the destructor is invoked on
    pub scrutinee: Rc<Term>,
    /// The destructor name
    pub id: Name,
    /// The type arguments instantiating the type parameters of the type
    pub type_args: TypeArgs,
    /// The arguments of the destructor
    pub args: Arguments,
    /// Type (inferred) of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Destructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Destructor {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        if (matches!(*self.scrutinee, Term::XVar(_))
            || matches!(*self.scrutinee, Term::Call(ref call) if call.args.entries.is_empty()))
            && (self.scrutinee.print_to_string(Some(cfg)).len() <= cfg.indent.cast_unsigned())
        {
            self.scrutinee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.type_args.print(cfg, alloc))
                .append(args.group())
        } else {
            self.scrutinee
                .print(cfg, alloc)
                .append(alloc.line_())
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.type_args.print(cfg, alloc))
                .append(args.group())
                .nest(cfg.indent)
                .align()
        }
    }
}

impl From<Destructor> for Term {
    fn from(value: Destructor) -> Self {
        Term::Destructor(value)
    }
}

impl Check for Destructor {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        // the name of the constructor in the symbol table for the instantiated data type
        let dtor_name = self.id.clone() + &self.type_args.print_to_string(None);
        let ty = match symbol_table.lookup_ty_for_dtor(&self.span, &dtor_name) {
            Ok(ty) => ty,
            // if there is no instance yet, we create an instance from the template
            Err(_) => symbol_table.lookup_ty_template_for_dtor(&self.id, &self.type_args)?,
        };

        self.scrutinee = self.scrutinee.check(symbol_table, context, &ty)?;

        match symbol_table.dtors.get(&dtor_name) {
            Some(signature) => {
                let (types, ret_ty) = signature.clone();

                self.args = check_args(&self.span, symbol_table, context, self.args, &types)?;

                check_equality(&self.span, symbol_table, expected, &ret_ty)?;

                self.ty = Some(expected.clone());
                Ok(self)
            }
            None => Err(Error::Undefined {
                span: Some(self.span),
                name: self.id.clone(),
            }),
        }
    }
}

impl Inference for Destructor {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut inference::VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error> {
        let mut constraints: Vec<(Ty, Ty)> = Vec::new();

        // creating a new type var to link the type of the current term to the future result after unification
        let new_type_var = var_name_generator.get_new_ty_var();
        self.ty = Some(new_type_var.clone());
        constraints.push((new_type_var, ty_var.clone()));

        let codata_type_name = match symbol_table.find_xdata_type_name(&self.id) {
            Some(type_name) => type_name,
            None => {
                return Err(Error::Undefined { span: Some(self.span), name: self.id.clone() });
            },
        };

        let (chirality, general_type_vars, _) = symbol_table.type_templates.get(&codata_type_name).unwrap();

        if chirality == &Polarity::Data {
                return Err(Error::ExpectedCovariableGotTerm { span: self.span });
        }

        // instanciating new type variables

        let mut type_var_mapping: HashMap<Name, Ty> = HashMap::new();

        if general_type_vars.bindings.len() == self.type_args.args.len() {
            // if the right amount of type arguments is given they are used

            for (type_var_name, given_ty )in general_type_vars.bindings.iter().zip(self.type_args.args.iter()) {
                type_var_mapping.insert(type_var_name.clone(), given_ty.clone());
            }
        } else if self.type_args.args.len() == 0 {
            // if no type Arguments are given, they are all replaced by variables,

            for type_var in &general_type_vars.bindings {
                type_var_mapping.insert(type_var.clone(), var_name_generator.get_new_ty_var());
            }
        } else {
            // if the wrong amount of type arguments are given, an error is returned

            return Err(Error::WrongNumberOfTypeArguments {
                span: Some(self.span),
                expected: general_type_vars.bindings.len(),
                got: self.type_args.args.len()
            });
        }
        

        // collecting the expected signature of the dtor
        let (mut arg_types, mut out_type) = match symbol_table.dtor_templates.get(&self.id) {
            Some((in_tys, out_tys)) => (in_tys.clone(), out_tys.clone()),
            None => {
                return Err(Error::Undefined { span: Some(self.span), name: self.id.clone() });
            }
        };

        // replacing the general type vars for instaciated ones

        let new_arg_types = arg_types.bindings.iter().map(|arg_ty| arg_ty.clone().subst_ty(&type_var_mapping)).collect();
        arg_types.bindings = new_arg_types;

        out_type = out_type.subst_ty(&type_var_mapping);

        // putting together the instantiated and expected type for the scrutinee and creating the constraints for it
        let scrutinee_type_args: Vec<Ty> = general_type_vars.bindings.iter().map(|binding| type_var_mapping.get(binding).unwrap()).cloned().collect();
        let scrutinee_type = Ty::mk_decl(&codata_type_name, TypeArgs { span: None, args: scrutinee_type_args });

        constraints.append(&mut self.scrutinee.constraint_equations(symbol_table, context, var_name_generator, scrutinee_type)?);

        constraints.append(&mut args_constraint_equations(&mut self.args, &arg_types, symbol_table, context, var_name_generator, self.span)?);

        constraints.push((ty_var, out_type));

        Ok(constraints)
    }


    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable
    ) -> Result<(), Error> {
        self.scrutinee.insert_inferred_type(mappings, symbol_table)?;
        
        for ty in &mut self.type_args.args {
            ty.mut_subst_ty(mappings);
            ty.check(&Some(self.span), symbol_table)?;
        }

        args_insert_inferred_type(&mut self.args, mappings, symbol_table)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span.clone()), symbol_table)
            },
            None => Ok(())
        }
    }
}

impl UsedBinders for Destructor {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.scrutinee.used_binders(used);
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod destructor_tests {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::inference::Inference;
    use crate::typing::inference::VarNameGenerator;
    use crate::typing::*;

    use std::rc::Rc;
    use std::vec;

    #[test]
    fn check_fst() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        let mut symbol_table = symbol_table_lpair();
        let result = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            args: vec![].into(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
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
            span: dummy_span(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![Lit::mk(1).into(), XVar::mk("a").into()].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: dummy_span(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![
                Lit::mk(1).into(),
                XVar {
                    span: dummy_span(),
                    var: "a".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Cns),
                }
                .into(),
            ]
            .into(),
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
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
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }


    #[test]
    fn inference_lpait_fst() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        let mut symbol_table = symbol_table_lpair();
        let mut term = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let scrutinee_type = Ty::mk_decl("LPair", TypeArgs::mk(vec![
            Ty::mk_ty_var("1"),
            Ty::mk_ty_var("2")
        ]));

        let expected = vec![
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            (Ty::mk_ty_var("3"), scrutinee_type.clone()),
            (scrutinee_type.clone(), Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]))),
            (Ty::mk_ty_var("x"), Ty::mk_ty_var("1"))
        ];

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
    }

    #[test]
    fn inference_lpait_fst_with_type_annotation() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        let mut symbol_table = symbol_table_lpair();
        let mut term = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let scrutinee_type = Ty::mk_decl("LPair", TypeArgs::mk(vec![
            Ty::mk_i64(),
            Ty::mk_i64()
        ]));

        let expected = vec![
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),

            (Ty::mk_ty_var("1"), scrutinee_type.clone()),
            (scrutinee_type.clone(), Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]))),

            (Ty::mk_ty_var("x"), Ty::mk_i64())
        ];

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("0")));
    }


    #[test]
    fn inference_ap() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        ctx.add_covar("a", Ty::mk_i64());
        let mut symbol_table = symbol_table_fun_template();
        let mut term = Destructor {
            span: dummy_span(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![]),
            args: vec![Lit::mk(1).into(), XVar::mk("a").into()].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let result = term.constraint_equations(&mut symbol_table, &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![
            // new type var
            (Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),

            // scrutinee
            (Ty::mk_ty_var("3"), Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("1"), Ty::mk_ty_var("2")]))),
            (Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("1"), Ty::mk_ty_var("2")])), Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]))),

            // argument 1
            (Ty::mk_ty_var("1"), Ty::mk_i64()),

            // argument 2,
            (Ty::mk_ty_var("2"), Ty::mk_i64()),

            //final type constraint
            (Ty::mk_ty_var("x"), Ty::mk_ty_var("2"))
        ];
        
        assert_eq!(result, expected)
    }

    #[test]
    fn inference_dtor_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_i64()])));
        let result = Destructor {
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .constraint_equations(&mut SymbolTable::default(), &ctx, &mut VarNameGenerator::new(), Ty::mk_ty_var("x"));
    
        assert!(result.is_err())
    }


    /// "x.head"
    fn example_1() -> Destructor {
        Destructor {
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            scrutinee: Rc::new(XVar::mk("x").into()),
            args: vec![].into(),
            ty: None,
        }
    }

    /// "x.head.head"
    fn example_2() -> Destructor {
        Destructor {
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            scrutinee: Rc::new(example_1().into()),
            args: vec![].into(),
            ty: None,
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(
            example_1().print_to_string(Default::default()),
            "x.head[i64]"
        )
    }

    #[test]
    fn display_2() {
        assert_eq!(
            example_2().print_to_string(Default::default()),
            "x.head[i64]\n    .head[i64]"
        )
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            scrutinee: Rc::new(XVar::mk("x").into()),
            args: vec![XVar::mk("y").into(), XVar::mk("z").into()].into(),
            ty: None,
        };
        let result = dest.print_to_string(Default::default());
        let expected = "x.fst[i64, i64](y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.head[i64]"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.head[i64].head[i64]"),
            Ok(example_2().into())
        );
    }
}
