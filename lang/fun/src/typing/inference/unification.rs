//! This module solves variational constraints. Solutions and conflicts carry the condition under
//! which they hold.

use std::collections::HashMap;

use derivative::Derivative;
use printer::Print;

use crate::{
    syntax::{Name, Ty},
    typing::{
        Error,
        inference::{ChoiceConflict, Condition, VariationalConstraint},
    },
};

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

/// This function adds a solution and, for every other solution of the same type variable that can
/// hold together with it, a constraint equating both types under the combined condition.
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
    // Unlike classic unification, solutions are never substituted into the remaining constraints,
    // since a solution only holds under its condition. Instead, a type variable collects all its
    // solutions, and two solutions that can hold together must have equal types, which
    // `add_solution` adds as a new constraint. All constraints consist of subterms of the original
    // types under finitely many conditions, and each is processed only once, so this terminates.
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
            }
            | VariationalConstraint::Equal {
                lhs: ty,
                rhs: Ty::TypeVar { name, .. },
                condition,
            } => {
                // one side is a type variable, so the other side is a solution for it, unless it
                // contains the variable itself (occurs check)
                if ty.type_var_names().contains(&name) {
                    conflicts.push(ChoiceConflict {
                        condition,
                        error: Error::InfiniteType {
                            span: ty.span(),
                            ty: ty.print_to_string(None),
                        },
                    });
                } else {
                    add_solution(
                        &mut constraints,
                        &mut solutions,
                        Solution::new(name.to_string(), ty, condition),
                    );
                }
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
                // two types, neither a type variable nor two declarations, which means a literal type and a declaration -> impossible to unify the equation
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

    #[test]
    fn unification_occurs_check() {
        let list_of_x = Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("x")]));
        let constraints = vec![VariationalConstraint::equal(
            Ty::mk_ty_var("x"),
            list_of_x.clone(),
        )];

        let (solutions, conflicts) = unify_variational(constraints);

        assert!(solutions.is_empty());
        assert_eq!(conflicts.len(), 1);
        assert_eq!(
            conflicts[0].error,
            Error::InfiniteType {
                span: None,
                ty: list_of_x.print_to_string(None),
            }
        );
    }
}
