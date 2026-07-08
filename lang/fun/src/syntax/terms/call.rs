//! This module defines the call of a top-level function in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Constraint;
use crate::typing::inference::ConstraintBank;
use crate::typing::inference::Inference;
use crate::typing::inference::args_constraint_equations;
use crate::typing::inference::args_insert_inferred_type;
use crate::typing::*;

use std::collections::HashMap;
use std::collections::HashSet;

/// This struct defines the call of a top-level function in Fun. It consists of the name of the
/// top-level function to call, the arguments, and after typechecking also the inferred type.
///
/// Example:
/// `fac(10)`, calls the top-level function `fac` with argument `10`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Call {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The name of the top-level function being called
    pub name: Name,
    /// The arguments
    pub args: Arguments,
    /// The (inferred) return type
    pub ret_ty: Option<Ty>,
    /// The potential choice identifier to resolve overloading
    pub choice_id: Option<u32>
}

impl OptTyped for Call {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Term {
    fn from(value: Call) -> Self {
        Term::Call(value)
    }
}

impl Inference for Call {
    fn gather_constraints(
            &mut self,
            constraint_bank: &mut ConstraintBank,
            context: &TypingContext,
            ty_var: Ty
        ) -> Result<(), Error> {
        match constraint_bank.symbol_table.variational_defs.get(&self.name) {
            Some(signatures) if signatures.is_empty() => {
                panic!("encountered a function definition with no signature")
            },
            Some(signatures) if signatures.len() == 1 => {
                // there is only one signature -> the function has no overloading, no need to add a variation variable
                let signature = signatures[0].clone();


                // adding a new type var as the type of the term for easier lookup after unification
                let new_type_var = constraint_bank.var_name_generator.get_new_ty_var();
                self.ret_ty = Some(new_type_var.clone());
                constraint_bank.constraints.push(Constraint::mk_only_ty(new_type_var, ty_var.clone()));

                let (types, ret_ty) = signature.clone();
                constraint_bank.constraints.push(Constraint::mk_only_ty(ty_var, ret_ty));

                args_constraint_equations(&mut self.args, &types, context, constraint_bank, self.span)?;

                Ok(())
            },
            Some(signatures) => {
                // there are more than one signatures for a function -> it is overloaded

                // adding a new type var as the type of the term for easier lookup after unification
                let new_type_var = constraint_bank.var_name_generator.get_new_ty_var();
                self.ret_ty = Some(new_type_var.clone());
                constraint_bank.constraints.push(Constraint::mk_only_ty(new_type_var, ty_var.clone()));

                // setting the choice id to resolve the overload later
                let new_choice_id = ConstraintBank::get_new_choice_id(
                    &mut constraint_bank.var_name_generator,
                    &mut constraint_bank.possible_choices,
                    signatures.len());
                
                self.choice_id = Some(new_choice_id);

                // the constraints are created for every choice, and marked with the choice made
                for (signature_idx, signature) in signatures.clone().iter().enumerate() {
                    let (mut types, ret_ty) = signature.clone();

                    // the return type is linked to the choice
                    constraint_bank.constraints.push(Constraint::mk_single_choice(ty_var.clone(), ret_ty, new_choice_id, signature_idx));

                    // the argument types are replaced by type variables to enable linking the choice to the argument types
                    for binding in types.bindings.iter_mut() {
                        let old_type = &binding.ty;
                        let new_type_var = constraint_bank.var_name_generator.get_new_ty_var();
                        constraint_bank.constraints.push(Constraint::mk_single_choice(old_type.clone(), new_type_var.clone(), new_choice_id, signature_idx));
                        binding.ty = new_type_var;
                    }

                    match args_constraint_equations(&mut self.args, &types, context, constraint_bank, self.span) {
                        Err(Error::WrongNumberOfArguments { span, expected, got }) => {
                            // The wrong number of Arguments Error only indicates that this version of the function won't work,
                            // others could still work, so the error is catched and marked as an impossible world
                            constraint_bank.constraints.push(Constraint::mk_impossible_world(new_choice_id, signature_idx, Error::WrongNumberOfArguments { span, expected, got }));
                        },
                        Err(other_err) => {
                            return Err(other_err);
                        },
                        _ => {}
                    }
                }

                Ok(())
            },
            None => Err(Error::Undefined {
                span: None,
                name: self.name.clone(),
            }),
        }

    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>
    ) -> Result<(), Error> {
        args_insert_inferred_type(&mut self.args, mappings, symbol_table, choices)?;
        if let Some(choice) = &self.choice_id {
            let index = choices.get(choice).expect("Although the function call had a choice_id, the choice id was not in the choices");
            // insert the individual name of the function, if it is overloaded
            self.name = symbol_table::build_unique_def_name(&self.name, index);
        }
        
        match &mut self.ret_ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            },
            None => panic!("The Type of the term {:?} is not set after type inference", self)
        }
    }
}

impl UsedBinders for Call {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::inference::Constraint;
    use crate::typing::inference::ConstraintBank;
    use crate::typing::inference::Inference;
    use crate::typing::*;


    #[test]
    fn inference_simple_def() {
        let mut symbol_table = SymbolTable::default();
        let mut typing_ctx = TypingContext::default();
        typing_ctx.add_var("x", Ty::mk_i64());
        symbol_table.variational_defs.insert("simple".to_owned(), vec![(typing_ctx, Ty::mk_ty_var("out_type"))]);
        
        let mut term = Call{
            span: dummy_span(),
            name: "simple".to_owned(),
            args: Arguments{
                entries: vec![Lit::mk(5).into()]
            },
            ret_ty: None,
            choice_id: None
        };

        let mut constraint_bank = ConstraintBank{
            symbol_table,
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &TypingContext::default(), Ty::mk_ty_var("x")).unwrap();

        let expected = vec![
            Constraint::mk_only_ty(Ty::mk_ty_var("0"), Ty::mk_ty_var("x")),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_ty_var("out_type")),

            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64())
        ];

        let ConstraintBank { constraints: result, .. } = constraint_bank;

        assert_eq!(result, expected);
        assert_eq!(term.ret_ty, Some(Ty::mk_ty_var("0")))
    }


    #[test]
    fn inference_mssing_def() {
        let mut term = Call{
            span: dummy_span(),
            name: "simple".to_owned(),
            args: Arguments{
                entries: vec![Lit::mk(5).into()]
            },
            ret_ty: None,
            choice_id: None
        };

        let mut constraint_bank = ConstraintBank{
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        let result = term.gather_constraints(&mut constraint_bank, &TypingContext::default(), Ty::mk_ty_var("x"));

        assert!(result.is_err_and(|e| matches!(e, Error::Undefined { name, .. } if name == "simple")))
    }

    fn example_simple() -> Call {
        Call {
            span: dummy_span(),
            name: "foo".to_string(),
            args: vec![].into(),
            ret_ty: None,
            choice_id: None
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "foo()"
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    fn example_extended() -> Call {
        Call {
            span: dummy_span(),
            name: "foo".to_string(),
            args: vec![Term::Lit(Lit::mk(2)).into(), XVar::mk("a").into()].into(),
            ret_ty: None,
            choice_id: None
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(
            example_extended().print_to_string(Default::default()),
            "foo(2, a)"
        )
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2, a)"), Ok(example_extended().into()));
    }
}
