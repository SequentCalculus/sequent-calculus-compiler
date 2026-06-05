use std::{collections::HashMap, rc::Rc};

use derivative::Derivative;
use miette::SourceSpan;

use crate::{syntax::{Arguments, Chirality::{Cns, Prd}, Name, Term, Ty, TypingContext}, typing::{Error, SymbolTable}};


pub trait Inference: Sized {

    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<Constraint>, Error>;

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<Name, usize>
    ) -> Result<(), Error>;
}

impl<T: Inference + Clone> Inference for Rc<T> {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<Constraint>, Error> {
        Rc::make_mut(self).constraint_equations(symbol_table, context, var_name_generator, ty_var)
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<Name, usize>
    ) -> Result<(), Error> {
        Rc::make_mut(self).insert_inferred_type(mappings, symbol_table, choices)
    }
}

impl<T: Inference> Inference for Option<T> {
    fn constraint_equations(
        &mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        var_name_generator: &mut VarNameGenerator,
        ty_var: Ty
    ) -> Result<Vec<Constraint>, Error> {
        match self {
            None => Ok(vec![]),
            Some(t ) => t.constraint_equations(symbol_table, context, var_name_generator, ty_var)
        }
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<Name, usize>
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.insert_inferred_type(mappings, symbol_table, choices),
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
) -> Result<Vec<Constraint>, Error> {

    let mut constraints: Vec<Constraint> = Vec::new();

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
                        constraints.push(Constraint::mk_only_ty(ty.clone(), found_ty.clone()));
                    } else {
                        let new_type_var = var_name_generator.get_new_ty_var();
                        variable.ty = Some(new_type_var.clone());
                        constraints.push(Constraint::mk_only_ty(new_type_var, found_ty.clone()));
                    }

                    constraints.push(Constraint::mk_only_ty(expected_type.ty.clone(), found_ty));
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
    symbol_table: &mut SymbolTable,
    choices: &HashMap<Name, usize>
) -> Result<(), Error> {
    for term in &mut args.entries {
        term.insert_inferred_type(mappings, symbol_table, choices)?;
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
        Ty::mk_ty_var(&name)
    }
}

impl Default for VarNameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// todo: make an incompatible choices struct with the Error explaining why this whould be impossible
pub type IncompatibleChoices = Vec<(Name, usize)>;



#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Constraint {
    Equality(Ty, Ty, HashMap<Name, usize>),
    ImpossibleWorld(IncompatibleChoices)
}

impl Constraint {
    pub fn mk_equality(a: Ty, b: Ty, choices: HashMap<Name, usize>) -> Self {
        Constraint::Equality(a, b, choices)
    }

    pub fn mk_only_ty(a: Ty, b: Ty) -> Self {
        Constraint::Equality(a, b, Default::default())
    }

    pub fn mk_single_choice(a: Ty, b: Ty, choice_name: Name, choice_number: usize) -> Self {
        let mut choices = HashMap::new();
        choices.insert(choice_name, choice_number);
        Constraint::Equality(a, b, choices)
    }

    pub fn mk_impossible_world(choice_name: Name, choice_number: usize) -> Self {
        let mut choices = Vec::new();
        choices.push((choice_name, choice_number));
        Constraint::ImpossibleWorld(choices)
    }

    pub fn add_choice(&mut self, choice_name: Name, choice_number: usize) {
        match self {
            Self::Equality(_, _, choices) => {
                choices.insert(choice_name, choice_number);
            },
            Self::ImpossibleWorld(_) => {}
        }
    }

    /// this function checks if a Solution can be combined/applied to this Constraint.
    fn in_same_choice_realm(&self, other: &Solution) -> bool {
        match self {
            Constraint::Equality(_, _, choices) => 
                // checking that all choices of the Solution are also in this constraint
                other.choices.iter().all(|(name, choice_idx)| choices.get(name) == Some(choice_idx)),            
            Constraint::ImpossibleWorld(_) => false,
        }        
    }
}

pub fn add_choice_to_list(constraints: &mut Vec<Constraint>, choice_name: Name, choice_number: usize) {
    for constraint in constraints {
        constraint.add_choice(choice_name.clone(), choice_number);
    }
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Solution {
    pub var_name: Name,
    pub ty: Ty,
    pub choices: HashMap<Name, usize>
}

impl Solution {
    fn new(var_name: Name, ty: Ty, choices: HashMap<String, usize>) -> Self {
        Solution { var_name, ty, choices }  
    }

    // the function is used in the tests    
    fn new_no_choice(var_name: Name, ty: Ty) -> Self {
        Solution { var_name, ty, choices: Default::default() }
    }

    pub fn get_only_solution(self) -> (Name, Ty) {
        (self.var_name, self.ty)
    }

    /// this function checks if another Solution can be combined/applied to this Solution.
    fn in_same_choice_realm(&self, other: &Self) -> bool {
        // checking that all choices of the other Solution are also in this Solution --> the other Solution can be applied to this one
        other.choices.iter().all(|(name, choice_idx)| self.choices.get(name) == Some(choice_idx))
    }
}

fn integrate_new_solution(equations: &mut Vec<Constraint>, solutions: &mut Vec<Solution>, new_solution: Solution) {
    // applying the new solution to all constraints
    for constraint in equations {
        if constraint.in_same_choice_realm(&new_solution) {
            match constraint {
                Constraint::Equality(ty1, ty2, _) => {
                    ty1.mut_subst_one_ty(&new_solution.var_name, &new_solution.ty);
                    ty2.mut_subst_one_ty(&new_solution.var_name, &new_solution.ty);
                },
                Constraint::ImpossibleWorld(_) => {}
            }
        }
    }

    // applying the new solution to all other solutions
    for solution in solutions.iter_mut() {
        if solution.in_same_choice_realm(&new_solution) {
            solution.ty.mut_subst_one_ty(&new_solution.var_name, &new_solution.ty);
        }
    }

    // adding the new solution to the solution list
    solutions.push(new_solution);
}

pub fn constraint_unification(mut equations: Vec<Constraint>) -> (Vec<Solution>, Vec<IncompatibleChoices>) {
    let mut solutions: Vec<Solution> = Vec::new();
    let mut conflicts: Vec<IncompatibleChoices> = Vec::new();
    let mut constraint_cache: Vec<Constraint> = Vec::new();

    while let Some(constraint) = equations.pop() {

        if constraint_cache.contains(&constraint) {
            continue;
        }

        constraint_cache.push(constraint.clone());

        match constraint {
            // two types that are the same have no value for the solution since x=x is trivial
            Constraint::Equality(ty1, ty2 , _) if ty1 == ty2 => {continue;},
            Constraint::Equality(Ty::TypeVar { name, .. }, ty, choices) => {
                // the first ty is a variable, so it can be added to the solutions
                integrate_new_solution(&mut equations, &mut solutions, Solution::new(name.to_string(), ty, choices));
            },
            Constraint::Equality(ty, Ty::TypeVar { name, .. }, choices) => {
                // the second ty is a variable, but not the first, so it is added "in reverse"
                integrate_new_solution(&mut equations, &mut solutions, Solution::new(name.to_string(), ty, choices));
            },
            Constraint::Equality(Ty::Decl { span: _ , name: name_l, type_args: type_args_l }, Ty::Decl {name: name_r, type_args: type_args_r, .. }, choices) => {
                if name_l == name_r {
                    // two matching (co-)datatypes in a constraint
                    if type_args_l.args.len() == type_args_r.args.len() {
                        for (ty_l, ty_r) in type_args_l.args.iter().zip(type_args_r.args.iter()) {
                            equations.push(Constraint::mk_equality(ty_l.clone(), ty_r.clone(), choices.clone()));
                        }
                    } else {
                        // theoretically impossible branch, where the type name is the same, but for some reason one Decl has more Type Arguments than the other
                        // this should already be covered by the constraint collection
                        panic!("Two instances of the same (co-)datatype have different number of arguments");
                    }
                } else {
                    // two different (co-)datatypes are in a constraint -> impossible to unify the equation
                    conflicts.push(choices.into_iter().collect());
                }
            },
            Constraint::Equality(_, _, choices) => {
                // two types, neither a type variable nor two declerations, which means a literal type and a declaration -> impossible to unify the equation
                conflicts.push(choices.into_iter().collect());
            },
            Constraint::ImpossibleWorld(choices) => {
                conflicts.push(choices);
            }
        };

    }

    (solutions, conflicts)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{syntax::{Ty, TypeArgs}, typing::inference::{Constraint, Solution, constraint_unification}};


    #[test]
    fn unification_test1() {
        let constraints = vec![
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64())
        ];

        let (solutions, conflicts) = constraint_unification(constraints);

        let expected: Vec<Solution> = vec![Solution::new_no_choice("x".to_string(), Ty::mk_i64())];

        assert_eq!(solutions, expected);
        assert!(conflicts.is_empty());
    }


    #[test]
    fn unification_test2() {
        let constraints = vec![
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("z"), Ty::mk_ty_var("meta_var 1")),
            Constraint::mk_only_ty(Ty::mk_ty_var("y"), Ty::mk_decl("Pair", TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_ty_var("z")]))),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let (solutions, conflicts) = constraint_unification(constraints);

        let expected = vec![
            Solution::new_no_choice("x".to_string(), Ty::mk_i64()),
            Solution::new_no_choice("y".to_string(), Ty::mk_decl("Pair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("meta_var 1")]))),
            Solution::new_no_choice("z".to_string(), Ty::mk_ty_var("meta_var 1"))
        ];
        
        assert_eq!(solutions, expected);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_decl_equation() {
        let constraints = vec![
            Constraint::mk_only_ty(Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")])), Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("x")])))
        ];

        let (solutions, conflicts) = constraint_unification(constraints);

        let expected = vec![
            Solution::new_no_choice("a".to_string(), Ty::mk_i64()),
            Solution::new_no_choice("b".to_string(), Ty::mk_ty_var("x"))
        ];

        assert_eq!(solutions, expected);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_impossible_constraint1() {
        let constraints = vec![
            Constraint::mk_equality(Ty::mk_i64(), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])), HashMap::from([("a".to_string(), 1)]))
        ];

        let (_, conflicts) = constraint_unification(constraints);

        let expected_conflict: Vec<Vec<(String, usize)>> = vec![vec![("a".to_string(), 1)]];

        assert_eq!(conflicts, expected_conflict);
    }

    #[test]
    fn unification_impossible_constraint2() {
        let constraints = vec![
            Constraint::mk_equality(Ty::mk_decl("Pair", TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")])), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])), HashMap::from([("a".to_string(), 2), ("b".to_string(), 5)]))
        ];

        let (_, conflicts) = constraint_unification(constraints);

        let expected_conflicts = vec![vec![("a".to_string(), 2), ("b".to_string(), 5)]];

        assert_eq!(conflicts, expected_conflicts);
    }


    #[test]
    fn unification_impossible_constraint3() {
        let constraints = vec![
            Constraint::mk_only_ty(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])), Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_decl("Optional", TypeArgs::mk(vec![Ty::mk_ty_var("a")]))])))
        ];

        let (_, conflicts) = constraint_unification(constraints);

        assert_eq!(conflicts, vec![vec![]]);
    }
}
