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
                    variable.chi = Some(Cns);

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
            arg.gather_constraints(constraint_bank, context, expected_type.ty.clone())?;
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
    fn _new_no_choice(var_name: Name, ty: Ty) -> Self {
        Solution {
            var_name,
            ty,
            choices: Default::default(),
        }
    }

    pub fn get_only_solution(self) -> (Name, Ty) {
        (self.var_name, self.ty)
    }

    fn get_new_constraints_with_choice(&self, new_solution: &Solution) -> Option<Constraint> {
        if self.choices.iter().all(|(choice_id, signature_id)| {
            new_solution
                .choices
                .get(choice_id)
                .is_none_or(|sig| sig == signature_id)
        }) {
            // if there are no conflicting choices the choices can be extended
            let mut combined_choice = self.choices.clone();
            combined_choice.extend(new_solution.choices.iter());

            Some(Constraint::Equality(
                new_solution.ty.clone(),
                self.ty.clone(),
                combined_choice,
            ))
        } else {
            // if there are conflicting choices, no new Constraint is created
            None
        }
    }
}

/// integrates a new [`Solution`] into the [`SolutionCache`] and [`Constraints`](Constraint)
/// by adding new constraints for solutions that have the same [`TypeVar`](Ty) as the new solution
fn integrate_new_solution(
    equations: &mut Vec<Constraint>,
    solutions: &mut SolutionCache,
    new_solution: Solution,
) {
    if let Some(related_solutions) = solutions.get(&new_solution.var_name) {
        // all solutions that have the same var_name as the new solution are selected
        let new_constraints = related_solutions
            .iter()
            .filter_map(|s| s.get_new_constraints_with_choice(&new_solution));
        // all solutions are converted to constraints with the new choices added

        equations.extend(new_constraints);
    }

    solutions.add_solution(new_solution);
}

pub fn constraint_unification(
    mut equations: Vec<Constraint>,
) -> (Vec<Solution>, Vec<IncompatibleChoices>) {
    let mut conflicts: Vec<IncompatibleChoices> = Vec::new();
    let mut constraint_cache: Vec<Constraint> = Vec::new();
    let mut solutions: SolutionCache = SolutionCache::new();

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
                    span: span_l,
                    name: name_l,
                    type_args: type_args_l,
                },
                Ty::Decl {
                    span: span_r,
                    name: name_r,
                    type_args: type_args_r,
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
                            "Two instances of the same (co-)datatype have different number of arguments. {}{:?}:{:?} = {}{:?}:{:?}",
                            name_l, type_args_l.args, span_l, name_r, type_args_r.args, span_r
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

    (solutions.all_solutions(), conflicts)
}

/// The [`SolutionCache`] can store solutions for multiple choices
pub struct SolutionCache {
    mapping: HashMap<Name, Vec<Solution>>,
}

impl SolutionCache {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    /// adding a new solution to the cache
    pub fn add_solution(&mut self, new_solution: Solution) {
        // adding an entry for the solution var name
        if let Some(entries) = self.mapping.get_mut(&new_solution.var_name) {
                entries.push(new_solution);
        } else {
            self.mapping
                .insert(new_solution.var_name.clone(), vec![new_solution]);
        }
    }

    pub fn get(&self, var_name: &Name) -> Option<&Vec<Solution>> {
        self.mapping.get(var_name)
    }

    pub fn all_solutions(self) -> Vec<Solution> {
        self.mapping.into_values().flatten().collect()
    }
}

impl Default for SolutionCache {
    fn default() -> Self {
        Self {
            mapping: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        syntax::{Ty, TypeArgs},
        typing::inference::{Constraint, Solution, SolutionCache, constraint_unification},
    };

    #[test]
    fn solution_cache_test1() {
        let mut solution_cache = SolutionCache::new();

        let solution_1 = Solution::_new_no_choice("a".to_string(), Ty::mk_i64());
        let solution_2 = Solution::_new_no_choice("b".to_string(), Ty::mk_ty_var("a"));
        let solution_3 = Solution::_new_no_choice("c".to_string(), Ty::mk_i64());
        let solution_4 =
            Solution::new("c".to_string(), Ty::mk_ty_var("4"), HashMap::from([(5, 4)]));

        solution_cache.add_solution(solution_1.clone());
        solution_cache.add_solution(solution_2.clone());
        solution_cache.add_solution(solution_3.clone());
        solution_cache.add_solution(solution_4.clone());

        let current_entries_a = solution_cache.get(&"a".to_string());
        let current_entries_b = solution_cache.get(&"b".to_string());
        let current_entries_c = solution_cache.get(&"c".to_string());

        let expected_entries_a = Some(&vec![solution_1]);

        let expected_entries_b = Some(&vec![solution_2]);

        let expected_entries_c = Some(&vec![solution_3, solution_4]);

        assert_eq!(current_entries_a, expected_entries_a);
        assert_eq!(current_entries_b, expected_entries_b);
        assert_eq!(current_entries_c, expected_entries_c);

        let all_solutions = solution_cache.all_solutions();
        let expected_solutions = vec![
            Solution::_new_no_choice("a".to_string(), Ty::mk_i64()),
            Solution::_new_no_choice("b".to_string(), Ty::mk_ty_var("a")),
            Solution::_new_no_choice("c".to_string(), Ty::mk_i64()),
            Solution::new("c".to_string(), Ty::mk_ty_var("4"), HashMap::from([(5, 4)])),
        ];

        // we need to check that it has the same elements, but not necessarily in the same order
        assert_eq!(all_solutions.len(), expected_solutions.len());
        assert!(expected_solutions.iter().all(|s| all_solutions.contains(s)))
    }

    #[test]
    fn unification_test1() {
        let constraints = vec![
            Constraint::mk_only_ty(Ty::mk_i64(), Ty::mk_i64()),
            Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let (solutions, conflicts) = constraint_unification(constraints);

        let expected: Vec<Solution> = vec![Solution::_new_no_choice("x".to_string(), Ty::mk_i64())];

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
            Solution::_new_no_choice("x".to_string(), Ty::mk_i64()),
            Solution::_new_no_choice(
                "y".to_string(),
                Ty::mk_decl(
                    "Pair",
                    TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_ty_var("z")]),
                ),
            ),
            Solution::_new_no_choice("z".to_string(), Ty::mk_ty_var("meta_var 1")),
        ];

        assert!(expected.iter().all(|s| solutions.contains(s)));
        assert_eq!(expected.len(), solutions.len());
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
            Solution::_new_no_choice("b".to_string(), Ty::mk_ty_var("x")),
            Solution::_new_no_choice("a".to_string(), Ty::mk_i64()),
        ];

        assert!(expected.iter().all(|s| solutions.contains(s)));
        assert_eq!(expected.len(), solutions.len());
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

        assert!(
            expected_conflicts[0]
                .iter()
                .all(|c| conflicts[0].contains(c))
        );
        assert_eq!(expected_conflicts.len(), conflicts.len());
        assert_eq!(expected_conflicts[0].len(), conflicts[0].len());
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
