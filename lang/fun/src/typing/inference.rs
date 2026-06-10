use std::{collections::HashMap, rc::Rc};

use derivative::Derivative;
use miette::SourceSpan;

use crate::{
    syntax::{
        Arguments,
        Chirality::{Cns, Prd},
        Name, Term, Ty, TypingContext,
    },
    typing::{Error, SymbolTable},
};

pub trait Inference: Sized {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error>;

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error>;
}

impl<T: Inference + Clone> Inference for Rc<T> {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        Rc::make_mut(self).gather_constraints(constraint_bank, context, ty_var)
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        Rc::make_mut(self).insert_inferred_type(mappings, symbol_table, choices)
    }
}

impl<T: Inference> Inference for Option<T> {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.gather_constraints(constraint_bank, context, ty_var),
        }
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.insert_inferred_type(mappings, symbol_table, choices),
        }
    }
}

pub struct ConstraintBank {
    pub symbol_table: SymbolTable,
    pub var_name_generator: VarNameGenerator,
    pub constraints: Vec<Constraint>,
    pub possible_choices: Vec<(u32, usize)>,
}

impl ConstraintBank {
    pub fn get_new_choice_id(
        name_generator: &mut VarNameGenerator,
        possible_choices: &mut Vec<(u32, usize)>,
        number_of_alternatives: usize,
    ) -> u32 {
        let new_id = name_generator.get_new_id();
        possible_choices.push((new_id, number_of_alternatives));
        new_id
    }
}

pub fn args_constraint_equations(
    args: &mut Arguments,
    types: &TypingContext,
    context: &TypingContext,
    constraint_bank: &mut ConstraintBank,
    span: SourceSpan,
) -> Result<(), Error> {
    if args.entries.len() != types.bindings.len() {
        return Err(Error::WrongNumberOfArguments {
            span,
            expected: types.bindings.len(),
            got: args.entries.len(),
        });
    }

    for (arg, expected_type) in args.entries.iter_mut().zip(types.bindings.iter()) {
        if expected_type.chi == Cns {
            match arg {
                Term::XVar(variable) => {
                    if variable.chi == Some(Prd) {
                        return Err(Error::ExpectedCovariableGotTerm {
                            span: variable.span,
                        });
                    }

                    let found_ty = context.lookup_covar(&variable.var, &variable.span)?;
                    if let Some(ty) = &variable.ty {
                        constraint_bank
                            .constraints
                            .push(Constraint::mk_only_ty(ty.clone(), found_ty.clone()));
                    } else {
                        let new_type_var = constraint_bank.var_name_generator.get_new_ty_var();
                        variable.ty = Some(new_type_var.clone());
                        constraint_bank
                            .constraints
                            .push(Constraint::mk_only_ty(new_type_var, found_ty.clone()));
                    }

                    constraint_bank
                        .constraints
                        .push(Constraint::mk_only_ty(expected_type.ty.clone(), found_ty));
                }
                _ => return Err(Error::ExpectedCovariableGotTerm { span }),
            }
        } else {
            arg.gather_constraints(constraint_bank, context, expected_type.ty.clone())?
        }
    }

    Ok(())
}

pub fn args_insert_inferred_type(
    args: &mut Arguments,
    mappings: &HashMap<Name, Ty>,
    symbol_table: &mut SymbolTable,
    choices: &HashMap<u32, usize>,
) -> Result<(), Error> {
    for term in &mut args.entries {
        term.insert_inferred_type(mappings, symbol_table, choices)?;
    }

    Ok(())
}

pub struct VarNameGenerator {
    internal_counter: u32,
}

impl VarNameGenerator {
    pub fn new() -> Self {
        VarNameGenerator {
            internal_counter: 0,
        }
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

    pub fn get_new_id(&mut self) -> u32 {
        let new_id = self.internal_counter;
        self.internal_counter += 1;
        new_id
    }
}

impl Default for VarNameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// The [`ChoiceRealm`] is an enumerable to represent the possibilities for comparing
/// the choices of a [`Solution`] with a [`Constraint`] or other [`Solution`]
enum ChoiceRealm {
    /// The two entities are in the same Choice Realm, the new [`Solution`] can be applied
    Same,
    /// The two entities are compatible, so a new [`Solution`] could be applied on a copy of the first entity
    Compatible,
    /// The choices are incompatible and the new [`Solution`] can't be applied
    Incompatible,
}

// todo: make an incompatible choices struct with the Error explaining why this whould be impossible
pub type IncompatibleChoices = Vec<(u32, usize)>;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Constraint {
    Equality(Ty, Ty, HashMap<u32, usize>),
    ImpossibleWorld(IncompatibleChoices),
}

impl Constraint {
    /// Creates an equality constraint, with a = b, based on the choices in choices
    pub fn mk_equality(a: Ty, b: Ty, choices: HashMap<u32, usize>) -> Self {
        Constraint::Equality(a, b, choices)
    }

    /// Creates an equality constraint, with a = b, without any choices necessary
    pub fn mk_only_ty(a: Ty, b: Ty) -> Self {
        Constraint::Equality(a, b, Default::default())
    }

    /// Creates an equality constraint with a = b based on a single choice
    pub fn mk_single_choice(a: Ty, b: Ty, choice_id: u32, signature_id: usize) -> Self {
        let mut choices = HashMap::new();
        choices.insert(choice_id, signature_id);
        Constraint::Equality(a, b, choices)
    }

    /// Creates an impossible world constraint for a choice that is generally not viable
    pub fn mk_impossible_world(choice_id: u32, signature_id: usize) -> Self {
        let mut choices = Vec::new();
        choices.push((choice_id, signature_id));
        Constraint::ImpossibleWorld(choices)
    }

    pub fn add_choice(&mut self, choice_id: u32, signature_id: usize) {
        match self {
            Self::Equality(_, _, choices) => {
                choices.insert(choice_id, signature_id);
            }
            Self::ImpossibleWorld(_) => {}
        }
    }

    /// this function checks if a Solution can be combined/applied to this Constraint.
    fn get_choice_realm(&self, other: &Solution) -> ChoiceRealm {
        let mut same_choices: usize = 0;
        match self {
            Constraint::Equality(_, _, choices) => {
                // checking that all choices of the Solution are also in this constraint

                for (choice_id, signature_id) in other.choices.iter() {
                    match choices.get(choice_id) {
                        Some(found_signature_id) if found_signature_id == signature_id => {
                            same_choices += 1;
                        }
                        Some(_) => {
                            return ChoiceRealm::Incompatible;
                        }
                        None => {}
                    }
                }

                if same_choices == other.choices.len() {
                    ChoiceRealm::Same
                } else {
                    ChoiceRealm::Compatible
                }
            }
            //other.choices.iter().all(|(choice_id, signature_id)| choices.get(choice_id) == Some(signature_id)),
            Constraint::ImpossibleWorld(_) => ChoiceRealm::Incompatible,
        }
    }

    /// applies a new solution to a [`Constraint`] if it is in the same ChoiceRealm
    /// or if it is in a compatible Realm, a copy with the new choices is returned
    fn apply_new_solution(&mut self, other: &Solution) -> Option<Self> {
        match self.get_choice_realm(other) {
            ChoiceRealm::Same => {
                match self {
                    Constraint::Equality(ty1, ty2, _) => {
                        ty1.mut_subst_one_ty(&other.var_name, &other.ty);
                        ty2.mut_subst_one_ty(&other.var_name, &other.ty);
                    }
                    Constraint::ImpossibleWorld(_) => {}
                }
                return None;
            }
            ChoiceRealm::Compatible => match self {
                Constraint::ImpossibleWorld(_) => None,
                Constraint::Equality(ty1, ty2, choices) => {
                    let mut new_choices = choices.clone();
                    new_choices.extend(other.choices.clone());

                    Some(Constraint::Equality(ty1.clone(), ty2.clone(), new_choices))
                }
            },
            ChoiceRealm::Incompatible => None,
        }
    }
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Solution {
    pub var_name: Name,
    pub ty: Ty,
    pub choices: HashMap<u32, usize>,
}

impl Solution {
    fn new(var_name: Name, ty: Ty, choices: HashMap<u32, usize>) -> Self {
        Solution {
            var_name,
            ty,
            choices,
        }
    }

    // the function is used in the tests
    fn new_no_choice(var_name: Name, ty: Ty) -> Self {
        Solution {
            var_name,
            ty,
            choices: Default::default(),
        }
    }

    pub fn get_only_solution(self) -> (Name, Ty) {
        (self.var_name, self.ty)
    }

    /// this function checks if another Solution can be combined/applied to this Solution.
    fn get_choice_realm(&self, other: &Self) -> ChoiceRealm {
        let mut same_choices: usize = 0;

        for (choice_id, signature_id) in other.choices.iter() {
            match self.choices.get(choice_id) {
                Some(found_signature_id) if found_signature_id == signature_id => {
                    same_choices += 1;
                }
                Some(_) => {
                    return ChoiceRealm::Incompatible;
                }
                None => {}
            }
        }

        if same_choices == other.choices.len() {
            ChoiceRealm::Same
        } else {
            ChoiceRealm::Compatible
        }
    }

    /// applies a new solution to a [`Constraint`] if it is in the same ChoiceRealm
    /// or if it is in a compatible Realm, a copy with the new choices is returned
    fn apply_new_solution(&mut self, other: &Self) -> Option<Self> {
        match self.get_choice_realm(other) {
            ChoiceRealm::Same => {
                self.ty.mut_subst_one_ty(&other.var_name, &other.ty);
                None
            }
            ChoiceRealm::Compatible => {
                    let mut new_choices = self.choices.clone();
                    new_choices.extend(other.choices.clone());

                    Some(Self { var_name: self.var_name.clone(), ty: self.ty.clone(), choices: new_choices })
            },
            ChoiceRealm::Incompatible => None,
        }
    }
}

fn integrate_new_solution(
    equations: &mut Vec<Constraint>,
    solutions: &mut Vec<Solution>,
    new_solution: Solution,
) { 
    let mut additional_constraints = Vec::new();
    for constraint in equations.iter_mut() {
        if let Some(new_constraint) = constraint.apply_new_solution(&new_solution) {
            additional_constraints.push(new_constraint);
        }
    }
    equations.extend(additional_constraints);

    
    let mut additional_solutions = Vec::new();
    for solution in solutions.iter_mut() {
        if let Some(new_solution) = solution.apply_new_solution(&new_solution) {
            additional_solutions.push(new_solution);
        }
    }
    solutions.extend(additional_solutions);


    solutions.push(new_solution);
}

pub fn constraint_unification(
    mut equations: Vec<Constraint>,
) -> (Vec<Solution>, Vec<IncompatibleChoices>) {
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
            Constraint::Equality(ty1, ty2, _) if ty1 == ty2 => {
                continue;
            }
            Constraint::Equality(Ty::TypeVar { name, .. }, ty, choices) => {
                // the first ty is a variable, so it can be added to the solutions
                integrate_new_solution(
                    &mut equations,
                    &mut solutions,
                    Solution::new(name.to_string(), ty, choices),
                );
            }
            Constraint::Equality(ty, Ty::TypeVar { name, .. }, choices) => {
                // the second ty is a variable, but not the first, so it is added "in reverse"
                integrate_new_solution(
                    &mut equations,
                    &mut solutions,
                    Solution::new(name.to_string(), ty, choices),
                );
            }
            Constraint::Equality(
                Ty::Decl {
                    span: _,
                    name: name_l,
                    type_args: type_args_l,
                },
                Ty::Decl {
                    name: name_r,
                    type_args: type_args_r,
                    ..
                },
                choices,
            ) => {
                if name_l == name_r {
                    // two matching (co-)datatypes in a constraint
                    if type_args_l.args.len() == type_args_r.args.len() {
                        for (ty_l, ty_r) in type_args_l.args.iter().zip(type_args_r.args.iter()) {
                            equations.push(Constraint::mk_equality(
                                ty_l.clone(),
                                ty_r.clone(),
                                choices.clone(),
                            ));
                        }
                    } else {
                        // theoretically impossible branch, where the type name is the same, but for some reason one Decl has more Type Arguments than the other
                        // this should already be covered by the constraint collection
                        panic!(
                            "Two instances of the same (co-)datatype have different number of arguments"
                        );
                    }
                } else {
                    // two different (co-)datatypes are in a constraint -> impossible to unify the equation
                    let impossible_world = choices
                        .into_iter()
                        .map(|(choice_id, signature_id)| (choice_id, signature_id))
                        .collect();
                    conflicts.push(impossible_world);
                }
            }
            Constraint::Equality(_, _, choices) => {
                // two types, neither a type variable nor two declerations, which means a literal type and a declaration -> impossible to unify the equation
                let impossible_world = choices
                    .into_iter()
                    .map(|(choice_id, signature_id)| (choice_id, signature_id))
                    .collect();
                conflicts.push(impossible_world);
            }
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

    use crate::{
        syntax::{Ty, TypeArgs},
        typing::inference::{Constraint, Solution, constraint_unification},
    };

    #[test]
    fn unification_test1() {
        let constraints = vec![
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
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
            Constraint::mk_only_ty(
                Ty::mk_ty_var("y"),
                Ty::mk_decl(
                    "Pair",
                    TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_ty_var("z")]),
                ),
            ),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let (solutions, conflicts) = constraint_unification(constraints);

        let expected = vec![
            Solution::new_no_choice("x".to_string(), Ty::mk_i64()),
            Solution::new_no_choice(
                "y".to_string(),
                Ty::mk_decl(
                    "Pair",
                    TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("meta_var 1")]),
                ),
            ),
            Solution::new_no_choice("z".to_string(), Ty::mk_ty_var("meta_var 1")),
        ];

        assert_eq!(solutions, expected);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_decl_equation() {
        let constraints = vec![Constraint::mk_only_ty(
            Ty::mk_decl(
                "Fun",
                TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")]),
            ),
            Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("x")])),
        )];

        let (solutions, conflicts) = constraint_unification(constraints);

        let expected = vec![
            Solution::new_no_choice("a".to_string(), Ty::mk_i64()),
            Solution::new_no_choice("b".to_string(), Ty::mk_ty_var("x")),
        ];

        assert_eq!(solutions, expected);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_impossible_constraint1() {
        let constraints = vec![Constraint::mk_equality(
            Ty::mk_i64(),
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])),
            HashMap::from([(5, 1)]),
        )];

        let (_, conflicts) = constraint_unification(constraints);

        let expected_conflict = vec![vec![(5, 1)]];

        assert_eq!(conflicts, expected_conflict);
    }

    #[test]
    fn unification_impossible_constraint2() {
        let constraints = vec![Constraint::mk_equality(
            Ty::mk_decl(
                "Pair",
                TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")]),
            ),
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])),
            HashMap::from([(3, 2), (16, 5)]),
        )];

        let (_, conflicts) = constraint_unification(constraints);

        let expected_conflicts = vec![vec![(3, 2), (16, 5)]];

        assert_eq!(conflicts, expected_conflicts);
    }

    #[test]
    fn unification_impossible_constraint3() {
        let constraints = vec![Constraint::mk_only_ty(
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            Ty::mk_decl(
                "List",
                TypeArgs::mk(vec![Ty::mk_decl(
                    "Optional",
                    TypeArgs::mk(vec![Ty::mk_ty_var("a")]),
                )]),
            ),
        )];

        let (_, conflicts) = constraint_unification(constraints);

        assert_eq!(conflicts, vec![vec![]]);
    }
}
