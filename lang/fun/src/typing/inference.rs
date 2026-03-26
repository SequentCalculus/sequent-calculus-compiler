use std::{collections::HashMap, rc::Rc};

use miette::SourceSpan;

use crate::{syntax::{Arguments, Chirality::{Cns, Prd}, Name, Term, Ty, TypeArgs, TypingContext, util::dummy_span}, typing::{Error, SymbolTable}};


pub trait Inference: Sized {

    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error>;

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable
    ) -> Result<(), Error>;
}

impl<T: Inference + Clone> Inference for Rc<T> {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error> {
        Rc::make_mut(self).constraint_equations(symbol_table, context, var_name_generator, ty_var)
    }

    fn insert_inferred_type(
            &mut self,
            mappings: &HashMap<Name, Ty>,
            symbol_table: &mut SymbolTable
        ) -> Result<(), Error> {
        Rc::make_mut(self).insert_inferred_type(mappings, symbol_table)
    }
}

impl<T: Inference> Inference for Option<T> {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<(Ty,Ty)>, Error> {
        match self {
            None => Ok(vec![]),
            Some(t ) => t.constraint_equations(symbol_table, context, var_name_generator, ty_var)
        }
    }

    fn insert_inferred_type(
            &mut self,
            mappings: &HashMap<Name, Ty>,
            symbol_table: &mut SymbolTable
        ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.insert_inferred_type(mappings, symbol_table),
        }
    }
}

pub fn args_constraint_equations(
    args: &mut Arguments,
    types: &TypingContext,
    symbol_table: &mut SymbolTable,
    context: &TypingContext,
    var_name_generator: &mut VarNameGenerator,
    span: SourceSpan
) -> Result<Vec<(Ty, Ty)>, Error> {

    let mut constraints: Vec<(Ty, Ty)> = Vec::new();

    if args.entries.len() != types.bindings.len() {
        return Err(Error::WrongNumberOfArguments {
            span,
            expected: types.bindings.len(),
            got: args.entries.len()
        });
    }

    for (arg, expected_type) in args.entries.iter_mut().zip(types.bindings.iter()) {
        if expected_type.chi == Cns {
            match arg {
                Term::XVar(variable) => {
                    if variable.chi == Some(Prd) {
                        return Err(Error::ExpectedCovariableGotTerm { span: variable.span });
                    }

                    let found_ty = context.lookup_covar(&variable.var, &variable.span)?;
                    if let Some(ty) = &variable.ty {
                        constraints.push((ty.clone(), found_ty.clone()));
                    }

                    constraints.push((expected_type.ty.clone(), found_ty));
                },
                _ => return Err(Error::ExpectedCovariableGotTerm { span }),
            }
        } else {
            constraints.append(&mut arg.constraint_equations(symbol_table, context, var_name_generator, expected_type.ty.clone())?);
        }
    }

    Ok(constraints)
}

pub fn args_insert_inferred_type(
    args: &mut Arguments,
    mappings: &HashMap<Name, Ty>,
    symbol_table: &mut SymbolTable
) -> Result<(), Error> {
    for term in &mut args.entries {
        term.insert_inferred_type(mappings, symbol_table)?;
    }

    Ok(())
}

pub struct VarNameGenerator {
    internal_counter: u32
}

impl VarNameGenerator {
    pub fn new() -> Self {
        VarNameGenerator { internal_counter: 0 }
    }

    pub fn get_new_name(&mut self) -> String {
        let new_name = self.internal_counter.to_string();
        self.internal_counter += 1;
        new_name
    }

    pub fn get_new_ty_var(&mut self) -> Ty {
        let name = self.get_new_name();
        Ty::Decl { span: None, name, type_args: TypeArgs::mk(vec![]) }
    }
}

impl Default for VarNameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn constraint_unification(mut equations: Vec<(Ty, Ty)>) -> Result<HashMap<Name, Ty>, Error> {
    let mut type_mapping: HashMap<Name, Ty> = HashMap::new();
    
    while let Some(constraint) = &mut equations.pop() {
        if constraint.0 == constraint.1 {
            // the constraint is irrevelant, since it declares "x=x"
            continue;
        }

        let maybe_new_mapping = match constraint {
            (Ty::I64 { .. }, Ty::I64 { .. }) => {continue;},
            (Ty::Decl { name, type_args, .. }, ty) if type_args.args.is_empty() => {
                // the first ty is a variable, so it can be added to the mapping
                Some(HashMap::from([
                    (name.to_string(), ty.clone())
                ]))
            },
            (ty, Ty::Decl { name, type_args, .. }) if type_args.args.is_empty() => {
                // the second ty is a variable, but not the first, so it is insert "in reverse"
                Some(HashMap::from([
                    (name.to_string(), ty.clone())
                ]))
            },
            (Ty::Decl { span: span_l , name: name_l, type_args: type_args_l }, Ty::Decl {name: name_r, type_args: type_args_r, .. }) => {
                if name_l == name_r {
                    // two matching (co-)datatypes in a constraint
                    if type_args_l.args.len() == type_args_r.args.len() {
                        for (ty_l, ty_r) in type_args_l.args.iter().zip(type_args_r.args.iter()) {
                            equations.push((ty_l.clone(), ty_r.clone()));
                        }
                        None
                    } else {
                        // theoretically impossible branch, where the type name is the same, but for some reason one Decl has more Type Arguments than the other
                        // this should already be covered by the constraint collection
                        return Err(Error::WrongNumberOfTypeArguments { span: *span_l, expected: type_args_l.args.len(), got: type_args_r.args.len() });
                    }
                } else {
                    // two different (co-)datatypes are in a constraint -> impossible to unify the equation
                    return Err(Error::ConflictingTypeConstraints { span_l: span_l.unwrap_or(dummy_span()), expected_type_l: name_l.to_string(), expected_type_r: name_r.to_string()});
                }
            },
            (ty_l, ty_r) => {
                // two types, neither a type variable and also not two declerations, which means a literal type and a declaration -> impossible to unify the equation
                let span = match ty_l {
                    Ty::I64 { span } => span,
                    Ty::Decl { span,..} => span
                };
                return Err(Error::ConflictingTypeConstraints { span_l: span.unwrap_or(dummy_span()), expected_type_l: ty_l.to_string(), expected_type_r: ty_r.to_string()});
            }
        };

        if let Some(new_mapping) = maybe_new_mapping {
            // if a new mapping was found it is applied to all constraints
            for (ty_l, ty_r) in &mut equations {
                ty_l.mut_subst_ty(&new_mapping);
                ty_r.mut_subst_ty(&new_mapping);
            }

            type_mapping.extend(new_mapping);
        }
    }

    let old_type_mapping = type_mapping.clone();

    // finally the transitive hull of the mappings are used to get all final results in the mappings
    for ty in type_mapping.values_mut() {
        while ty.collect_var_names().iter().any(|name| old_type_mapping.contains_key(name)) {
            ty.mut_subst_ty(&old_type_mapping);
        }
    }

    Ok(type_mapping)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{syntax::{Ty, TypeArgs}, typing::inference::constraint_unification};


    #[test]
    fn unification_test1() {
        let constraints = vec![
            (Ty::mk_i64(), Ty::mk_i64()),
            (Ty::mk_ty_var("x"), Ty::mk_i64())
        ];

        let result = constraint_unification(constraints).unwrap();

        let mut expected = HashMap::new();
        expected.insert("x".to_string(), Ty::mk_i64());

        assert_eq!(result, expected);
    }


    #[test]
    fn unification_test2() {
        let constraints = vec![
            (Ty::mk_i64(), Ty::mk_i64()),
            (Ty::mk_ty_var("z"), Ty::mk_ty_var("meta_var 1")),
            (Ty::mk_ty_var("y"), Ty::mk_decl("Pair", TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_ty_var("z")]))),
            (Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let result = constraint_unification(constraints).unwrap();

        let expected = HashMap::from([
            ("x".to_string(), Ty::mk_i64()),
            ("y".to_string(), Ty::mk_decl("Pair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("meta_var 1")]))),
            ("z".to_string(), Ty::mk_ty_var("meta_var 1"))
        ]);
        
        assert_eq!(result, expected);
    }

    #[test]
    /// testing that the order of constraints should not affect the final result
    fn unification_cummutative() {
        let constraints1 = vec![
            (Ty::mk_ty_var("final_type"), Ty::mk_decl("ComplexType", TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_i64(), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("y")]))]))),
            (Ty::mk_ty_var("x"), Ty::mk_ty_var("a")),
            (Ty::mk_ty_var("a"), Ty::mk_i64()),
            (Ty::mk_ty_var("y"), Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("x")])))
        ];

        let mapping1 = constraint_unification(constraints1).unwrap();

        let constraints2 = vec![
            (Ty::mk_ty_var("x"), Ty::mk_ty_var("a")),
            (Ty::mk_ty_var("y"), Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("x")]))),
            (Ty::mk_ty_var("a"), Ty::mk_i64()),
            (Ty::mk_ty_var("final_type"), Ty::mk_decl("ComplexType", TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_i64(), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("y")]))]))),            
        ];

        let mapping2 = constraint_unification(constraints2).unwrap();

        assert_eq!(mapping1.get("final_type"), mapping2.get("final_type"));
    }

    #[test]
    fn unification_decl_equation() {
        let constraints = vec![
            (Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")])), Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("x")])))
        ];

        let result = constraint_unification(constraints).unwrap();

        let expected = HashMap::from([
            ("a".to_string(), Ty::mk_i64()),
            ("b".to_string(), Ty::mk_ty_var("x"))
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn unification_impossible_constraint1() {
        let constraints = vec![
            (Ty::mk_i64(), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])))
        ];

        let result = constraint_unification(constraints);

        assert!(result.is_err());
    }

    #[test]
    fn unification_impossible_constraint2() {
        let constraints = vec![
            (Ty::mk_decl("Pair", TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")])), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])))
        ];

        let result = constraint_unification(constraints);

        assert!(result.is_err());
    }


    #[test]
    fn unification_impossible_constraint3() {
        let constraints = vec![
            (Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_decl("Optional", TypeArgs::mk(vec![Ty::mk_ty_var("a")]))])))
        ];

        let result = constraint_unification(constraints);

        assert!(result.is_err());
    }
}