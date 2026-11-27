use std::{collections::HashMap, rc::Rc};

use derivative::Derivative;
use miette::SourceSpan;
use printer::Print;

use crate::{
    syntax::{
        Arguments,
        Chirality::{Cns, Prd},
        Name, Term, Ty, TypingContext,
    },
    typing::{Error, SymbolTable},
};

/// Identifies one overloaded call site, i.e., one choice between the overloads of a function.
pub type ChoiceId = u32;

/// Selects one alternative of a choice, i.e., the index of one overload of the called function.
pub type Alternative = usize;

/// A partial selection of alternatives under which a constraint or solution holds. A constraint
/// with an empty condition holds in every world.
pub type Condition = HashMap<ChoiceId, Alternative>;

/// A complete selection of alternatives, one for each choice of the program.
pub type World = HashMap<ChoiceId, Alternative>;

/// Maps the names of type variables to the types inferred for them.
pub type TypeVarMapping = HashMap<Name, Ty>;

/// A choice between the overloads of a function, created for one overloaded call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverloadChoice {
    /// The identifier of the choice
    pub id: ChoiceId,
    /// The number of alternatives, i.e., the number of overloads of the called function
    pub alternatives: usize,
}

/// This trait provides type inference for the terms of Fun. Inference happens in two passes: the
/// first collects the constraints of all terms, the second inserts the types computed from the
/// solution of these constraints.
pub trait TypeInference: Sized {
    /// This function collects the constraints for a term in the given typing context.
    /// - `state` is the inference state, which receives the constraints and choices.
    /// - `context` is the typing context in which the term is typed.
    /// - `expected_ty` is the type the term must have, often a fresh type variable.
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error>;

    /// This function replaces the type variables in the annotations of a term with the inferred
    /// types and resolves overloaded calls according to the selected world.
    /// - `type_vars` maps type variables to their inferred types.
    /// - `symbol_table` is used to create the monomorphic instances of the inferred types.
    /// - `world` is the selected alternative for each choice.
    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error>;
}

impl<T: TypeInference + Clone> TypeInference for Rc<T> {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        Rc::make_mut(self).collect_constraints(state, context, expected_ty)
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        Rc::make_mut(self).insert_inferred_types(type_vars, symbol_table, world)
    }
}

impl<T: TypeInference> TypeInference for Option<T> {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.collect_constraints(state, context, expected_ty),
        }
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.insert_inferred_types(type_vars, symbol_table, world),
        }
    }
}

/// This struct holds everything type inference accumulates while collecting constraints.
pub struct TypeInferenceState {
    /// The symbol table of the program
    pub symbol_table: SymbolTable,
    /// The generator for fresh type variables and choice identifiers
    pub fresh_names: FreshNames,
    /// The constraints collected so far
    pub constraints: Vec<VariationalConstraint>,
    /// The choices created so far, one for each overloaded call site
    pub choices: Vec<OverloadChoice>,
}

impl TypeInferenceState {
    /// This function creates a new choice with the given number of alternatives and returns its
    /// identifier.
    pub fn new_choice(&mut self, alternatives: usize) -> ChoiceId {
        let id = self.fresh_names.fresh_id();
        self.choices.push(OverloadChoice { id, alternatives });
        id
    }
}

/// This function collects the constraints for the arguments of a call, constructor or destructor.
/// Producer arguments must have the type of their parameter, covariable arguments are looked up in
/// the context and must have the type of their consumer parameter.
/// - `args` are the arguments.
/// - `types` are the parameters the arguments are passed to.
/// - `context` is the typing context of the arguments.
/// - `state` is the inference state.
/// - `span` is the source location used for arity errors.
pub fn collect_arg_constraints(
    args: &mut Arguments,
    types: &TypingContext,
    context: &TypingContext,
    state: &mut TypeInferenceState,
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
                        state
                            .constraints
                            .push(VariationalConstraint::equal(ty.clone(), found_ty.clone()));
                    } else {
                        let new_type_var = state.fresh_names.fresh_ty_var();
                        variable.ty = Some(new_type_var.clone());
                        state
                            .constraints
                            .push(VariationalConstraint::equal(new_type_var, found_ty.clone()));
                    }

                    state.constraints.push(VariationalConstraint::equal(
                        expected_type.ty.clone(),
                        found_ty,
                    ));
                }
                _ => return Err(Error::ExpectedCovariableGotTerm { span }),
            }
        } else {
            arg.collect_constraints(state, context, expected_type.ty.clone())?;
        }
    }

    Ok(())
}

/// This function inserts the inferred types into all arguments, see
/// [`TypeInference::insert_inferred_types`].
pub fn insert_arg_types(
    args: &mut Arguments,
    type_vars: &TypeVarMapping,
    symbol_table: &mut SymbolTable,
    world: &World,
) -> Result<(), Error> {
    for term in &mut args.entries {
        term.insert_inferred_types(type_vars, symbol_table, world)?;
    }

    Ok(())
}

/// This struct generates fresh names for type variables and fresh choice identifiers. Both share
/// one counter, so all generated names and identifiers are distinct.
pub struct FreshNames {
    internal_counter: u32,
}

impl FreshNames {
    /// This function creates a new generator starting at zero.
    pub fn new() -> Self {
        FreshNames {
            internal_counter: 0,
        }
    }

    /// This function returns a fresh name.
    fn fresh_name(&mut self) -> String {
        let new_name = self.internal_counter.to_string();
        self.internal_counter += 1;
        new_name
    }

    /// This function returns a fresh type variable.
    pub fn fresh_ty_var(&mut self) -> Ty {
        let name = self.fresh_name();
        Ty::mk_ty_var(&name)
    }

    /// This function returns a fresh identifier.
    pub fn fresh_id(&mut self) -> u32 {
        let new_id = self.internal_counter;
        self.internal_counter += 1;
        new_id
    }
}

impl Default for FreshNames {
    fn default() -> Self {
        Self::new()
    }
}

/// A combination of alternatives that cannot be selected together, together with the error
/// explaining why.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct ChoiceConflict {
    /// The alternatives that cannot be selected together
    pub condition: Condition,
    /// The type error arising when these alternatives are selected
    pub error: Error,
}

/// A constraint of variational type inference. It only holds in the worlds satisfying its
/// condition.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum VariationalConstraint {
    /// The two types must be equal under the condition.
    Equal {
        lhs: Ty,
        rhs: Ty,
        condition: Condition,
    },
    /// The condition can never be satisfied.
    Impossible(ChoiceConflict),
}

impl VariationalConstraint {
    /// Creates an equality constraint, with lhs = rhs, based on the given condition
    pub fn equal_under(lhs: Ty, rhs: Ty, condition: Condition) -> Self {
        VariationalConstraint::Equal {
            lhs,
            rhs,
            condition,
        }
    }

    /// Creates an equality constraint, with lhs = rhs, holding in every world
    pub fn equal(lhs: Ty, rhs: Ty) -> Self {
        VariationalConstraint::equal_under(lhs, rhs, Condition::default())
    }

    /// Creates an equality constraint with lhs = rhs based on a single alternative of a choice
    pub fn equal_if(lhs: Ty, rhs: Ty, choice: ChoiceId, alternative: Alternative) -> Self {
        VariationalConstraint::equal_under(lhs, rhs, Condition::from([(choice, alternative)]))
    }

    /// Creates a constraint marking a single alternative of a choice as generally not viable
    pub fn impossible_if(choice: ChoiceId, alternative: Alternative, error: Error) -> Self {
        VariationalConstraint::Impossible(ChoiceConflict {
            condition: Condition::from([(choice, alternative)]),
            error,
        })
    }
}

/// A solution for a type variable, i.e., the type it must be equal to under a condition.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Solution {
    /// The name of the solved type variable
    pub var: Name,
    /// The type the variable must be equal to
    pub ty: Ty,
    /// The condition under which this solution holds
    pub condition: Condition,
}

impl Solution {
    /// This function creates a solution holding under the given condition.
    fn new(var: Name, ty: Ty, condition: Condition) -> Self {
        Solution { var, ty, condition }
    }

    /// This function creates a solution holding in every world.
    #[cfg(test)]
    fn unconditional(var: Name, ty: Ty) -> Self {
        Solution {
            var,
            ty,
            condition: Condition::default(),
        }
    }

    /// This function returns the solved variable and its type, dropping the condition.
    pub fn into_pair(self) -> (Name, Ty) {
        (self.var, self.ty)
    }

    /// This function returns the constraint that makes this solution consistent with another
    /// solution for the same type variable, i.e., that equates both types under the combined
    /// condition. If the conditions select different alternatives for the same choice, the
    /// solutions never hold together and no constraint is needed.
    fn consistency_constraint(&self, new_solution: &Solution) -> Option<VariationalConstraint> {
        if self.condition.iter().all(|(choice, alternative)| {
            new_solution
                .condition
                .get(choice)
                .is_none_or(|other| other == alternative)
        }) {
            // if there are no conflicting choices the conditions can be combined
            let mut combined_condition = self.condition.clone();
            combined_condition.extend(new_solution.condition.iter());

            Some(VariationalConstraint::equal_under(
                new_solution.ty.clone(),
                self.ty.clone(),
                combined_condition,
            ))
        } else {
            // if there are conflicting choices, no new constraint is created
            None
        }
    }
}

/// integrates a new [`Solution`] into the [`Solutions`] and [`VariationalConstraint`]s
/// by adding new constraints for solutions that have the same [`TypeVar`](Ty) as the new solution
fn add_solution(
    constraints: &mut Vec<VariationalConstraint>,
    solutions: &mut Solutions,
    new_solution: Solution,
) {
    if let Some(related_solutions) = solutions.get(&new_solution.var) {
        // all solutions that have the same var as the new solution are selected
        let new_constraints = related_solutions
            .iter()
            .filter_map(|s| s.consistency_constraint(&new_solution));
        // all solutions are converted to constraints with the combined conditions

        constraints.extend(new_constraints);
    }

    solutions.insert(new_solution);
}

/// This function solves variational constraints. It returns all solutions for type variables,
/// each holding under a condition, and all conflicts, i.e., the conditions under which the
/// constraints cannot be satisfied.
pub fn unify_variational(
    mut constraints: Vec<VariationalConstraint>,
) -> (Vec<Solution>, Vec<ChoiceConflict>) {
    let mut conflicts: Vec<ChoiceConflict> = Vec::new();
    let mut seen_constraints: Vec<VariationalConstraint> = Vec::new();
    let mut solutions: Solutions = Solutions::new();

    while let Some(constraint) = constraints.pop() {
        if seen_constraints.contains(&constraint) {
            continue;
        }

        seen_constraints.push(constraint.clone());

        match constraint {
            // two types that are the same have no value for the solution since x=x is trivial
            VariationalConstraint::Equal { lhs, rhs, .. } if lhs == rhs => {
                continue;
            }
            VariationalConstraint::Equal {
                lhs: Ty::TypeVar { name, .. },
                rhs: ty,
                condition,
            } => {
                // the first ty is a variable, so it can be added to the solutions
                add_solution(
                    &mut constraints,
                    &mut solutions,
                    Solution::new(name.to_string(), ty, condition),
                );
            }
            VariationalConstraint::Equal {
                lhs: ty,
                rhs: Ty::TypeVar { name, .. },
                condition,
            } => {
                // the second ty is a variable, but not the first, so it is added "in reverse"
                add_solution(
                    &mut constraints,
                    &mut solutions,
                    Solution::new(name.to_string(), ty, condition),
                );
            }
            VariationalConstraint::Equal {
                lhs:
                    Ty::Decl {
                        span: span_l,
                        name: name_l,
                        type_args: type_args_l,
                    },
                rhs:
                    Ty::Decl {
                        span: span_r,
                        name: name_r,
                        type_args: type_args_r,
                    },
                condition,
            } => {
                if name_l == name_r {
                    // two matching (co-)datatypes in a constraint
                    if type_args_l.args.len() == type_args_r.args.len() {
                        for (ty_l, ty_r) in type_args_l.args.iter().zip(type_args_r.args.iter()) {
                            constraints.push(VariationalConstraint::equal_under(
                                ty_l.clone(),
                                ty_r.clone(),
                                condition.clone(),
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
                    let mut lhs = name_l;
                    lhs.push_str(&type_args_l.print_to_string(None));

                    let mut rhs = name_r;
                    rhs.push_str(&type_args_r.print_to_string(None));

                    let best_span = if let Some(span) = span_l {
                        Some(span)
                    } else {
                        span_r
                    };

                    conflicts.push(ChoiceConflict {
                        condition,
                        error: Error::ConflictingTypes {
                            span: best_span,
                            lhs,
                            rhs,
                        },
                    });
                }
            }
            VariationalConstraint::Equal {
                lhs,
                rhs,
                condition,
            } => {
                // two types, neither a type variable nor two declerations, which means a literal type and a declaration -> impossible to unify the equation
                let best_span = if let Some(span) = lhs.span() {
                    Some(span)
                } else {
                    rhs.span()
                };

                conflicts.push(ChoiceConflict {
                    condition,
                    error: Error::ConflictingTypes {
                        span: best_span,
                        lhs: lhs.print_to_string(None),
                        rhs: rhs.print_to_string(None),
                    },
                });
            }
            VariationalConstraint::Impossible(conflict) => {
                conflicts.push(conflict);
            }
        };
    }

    (solutions.into_vec(), conflicts)
}

/// The [`Solutions`] store all solutions found so far, grouped by the solved type variable. A type
/// variable can have several solutions under different conditions.
#[derive(Default)]
pub struct Solutions {
    by_var: HashMap<Name, Vec<Solution>>,
}

impl Solutions {
    /// This function creates an empty store.
    pub fn new() -> Self {
        Self {
            by_var: HashMap::new(),
        }
    }

    /// adding a new solution to the store
    pub fn insert(&mut self, new_solution: Solution) {
        // adding an entry for the solution var name
        if let Some(entries) = self.by_var.get_mut(&new_solution.var) {
            entries.push(new_solution);
        } else {
            self.by_var
                .insert(new_solution.var.clone(), vec![new_solution]);
        }
    }

    /// This function returns all solutions for the given type variable.
    pub fn get(&self, var: &Name) -> Option<&Vec<Solution>> {
        self.by_var.get(var)
    }

    /// This function returns all solutions for all type variables.
    pub fn into_vec(self) -> Vec<Solution> {
        self.by_var.into_values().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::typing::Error;
    use crate::{
        syntax::{Ty, TypeArgs},
        typing::inference::{
            ChoiceConflict, Condition, Solution, Solutions, VariationalConstraint,
            unify_variational,
        },
    };

    #[test]
    fn solutions_test1() {
        let mut solutions = Solutions::new();

        let solution_1 = Solution::unconditional("a".to_string(), Ty::mk_i64());
        let solution_2 = Solution::unconditional("b".to_string(), Ty::mk_ty_var("a"));
        let solution_3 = Solution::unconditional("c".to_string(), Ty::mk_i64());
        let solution_4 = Solution::new(
            "c".to_string(),
            Ty::mk_ty_var("4"),
            Condition::from([(5, 4)]),
        );

        solutions.insert(solution_1.clone());
        solutions.insert(solution_2.clone());
        solutions.insert(solution_3.clone());
        solutions.insert(solution_4.clone());

        let current_entries_a = solutions.get(&"a".to_string());
        let current_entries_b = solutions.get(&"b".to_string());
        let current_entries_c = solutions.get(&"c".to_string());

        let expected_entries_a = Some(&vec![solution_1]);

        let expected_entries_b = Some(&vec![solution_2]);

        let expected_entries_c = Some(&vec![solution_3, solution_4]);

        assert_eq!(current_entries_a, expected_entries_a);
        assert_eq!(current_entries_b, expected_entries_b);
        assert_eq!(current_entries_c, expected_entries_c);

        let all_solutions = solutions.into_vec();
        let expected_solutions = vec![
            Solution::unconditional("a".to_string(), Ty::mk_i64()),
            Solution::unconditional("b".to_string(), Ty::mk_ty_var("a")),
            Solution::unconditional("c".to_string(), Ty::mk_i64()),
            Solution::new(
                "c".to_string(),
                Ty::mk_ty_var("4"),
                Condition::from([(5, 4)]),
            ),
        ];

        // we need to check that it has the same elements, but not necessarily in the same order
        assert_eq!(all_solutions.len(), expected_solutions.len());
        assert!(expected_solutions.iter().all(|s| all_solutions.contains(s)))
    }

    #[test]
    fn unification_test1() {
        let constraints = vec![
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let (solutions, conflicts) = unify_variational(constraints);

        let expected: Vec<Solution> = vec![Solution::unconditional("x".to_string(), Ty::mk_i64())];

        assert_eq!(solutions, expected);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_test2() {
        let constraints = vec![
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("z"), Ty::mk_ty_var("meta_var 1")),
            VariationalConstraint::equal(
                Ty::mk_ty_var("y"),
                Ty::mk_decl(
                    "Pair",
                    TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_ty_var("z")]),
                ),
            ),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let (solutions, conflicts) = unify_variational(constraints);

        let expected = vec![
            Solution::unconditional("x".to_string(), Ty::mk_i64()),
            Solution::unconditional(
                "y".to_string(),
                Ty::mk_decl(
                    "Pair",
                    TypeArgs::mk(vec![Ty::mk_ty_var("x"), Ty::mk_ty_var("z")]),
                ),
            ),
            Solution::unconditional("z".to_string(), Ty::mk_ty_var("meta_var 1")),
        ];

        assert!(expected.iter().all(|s| solutions.contains(s)));
        assert_eq!(expected.len(), solutions.len());
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_decl_equation() {
        let constraints = vec![VariationalConstraint::equal(
            Ty::mk_decl(
                "Fun",
                TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")]),
            ),
            Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_ty_var("x")])),
        )];

        let (solutions, conflicts) = unify_variational(constraints);

        let expected = vec![
            Solution::unconditional("b".to_string(), Ty::mk_ty_var("x")),
            Solution::unconditional("a".to_string(), Ty::mk_i64()),
        ];

        assert!(expected.iter().all(|s| solutions.contains(s)));
        assert_eq!(expected.len(), solutions.len());
        assert!(conflicts.is_empty());
    }

    #[test]
    fn unification_impossible_constraint1() {
        let constraints = vec![VariationalConstraint::equal_under(
            Ty::mk_i64(),
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])),
            Condition::from([(5, 1)]),
        )];

        let (_, conflicts) = unify_variational(constraints);

        let expected_conflict = vec![ChoiceConflict {
            condition: Condition::from([(5, 1)]),
            error: Error::ConflictingTypes {
                span: None,
                lhs: Ty::mk_i64().print_to_string(None),
                rhs: Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")]))
                    .print_to_string(None),
            },
        }];

        assert_eq!(conflicts, expected_conflict);
    }

    #[test]
    fn unification_impossible_constraint2() {
        let constraints = vec![VariationalConstraint::equal_under(
            Ty::mk_decl(
                "Pair",
                TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")]),
            ),
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])),
            Condition::from([(3, 2), (16, 5)]),
        )];

        let (_, mut conflicts) = unify_variational(constraints);

        let expected_error = Error::ConflictingTypes {
            span: None,
            lhs: Ty::mk_decl(
                "Pair",
                TypeArgs::mk(vec![Ty::mk_ty_var("a"), Ty::mk_ty_var("b")]),
            )
            .print_to_string(None),
            rhs: Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("a")])).print_to_string(None),
        };

        let expected_condition = Condition::from([(3, 2), (16, 5)]);

        assert_eq!(conflicts.len(), 1);

        let ChoiceConflict { condition, error } = conflicts.swap_remove(0);

        assert_eq!(expected_condition, condition);
        assert_eq!(expected_error, error);
    }

    #[test]
    fn unification_impossible_constraint3() {
        let constraints = vec![VariationalConstraint::equal(
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            Ty::mk_decl(
                "List",
                TypeArgs::mk(vec![Ty::mk_decl(
                    "Optional",
                    TypeArgs::mk(vec![Ty::mk_ty_var("a")]),
                )]),
            ),
        )];

        let (_, mut conflicts) = unify_variational(constraints);

        let ChoiceConflict { condition, error } = conflicts.swap_remove(0);

        let expected_error = Error::ConflictingTypes {
            span: None,
            lhs: Ty::mk_i64().print_to_string(None),
            rhs: Ty::mk_decl("Optional", TypeArgs::mk(vec![Ty::mk_ty_var("a")]))
                .print_to_string(None),
        };

        assert!(condition.is_empty());
        assert_eq!(expected_error, error);
    }
}
