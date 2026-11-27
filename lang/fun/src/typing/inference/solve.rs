//! This module combines the phases of type inference that follow the collection of constraints:
//! unification, overload resolution and the construction of the final mapping of type variables.

use printer::Print;

use crate::{
    syntax::{Name, Ty},
    typing::{
        Error,
        inference::{
            OverloadChoice, Solution, TypeVarMapping, VariationalConstraint, World, select_world,
            unify_variational,
        },
    },
};

/// This function solves the collected constraints and resolves overloading. It returns the
/// inferred type of every type variable and the selected world.
/// - `constraints` are the constraints collected from the program.
/// - `choices` are the choices of all overloaded call sites.
pub fn solve_inference_constraints(
    constraints: Vec<VariationalConstraint>,
    choices: &[OverloadChoice],
) -> Result<(TypeVarMapping, World), Error> {
    let (solutions, conflicts) = unify_variational(constraints);

    // a conflict without condition holds in every world, so the program has a type error no
    // matter how overloading is resolved
    if let Some(conflict) = conflicts
        .iter()
        .find(|conflict| conflict.condition.is_empty())
    {
        return Err(conflict.error.clone());
    }

    // without choices there is only the empty world, and all conflicts were unconditional
    let world = if choices.is_empty() {
        World::new()
    } else {
        select_world(choices, conflicts)?
    };

    let solutions = solutions_in_world(solutions, &world);
    let type_vars = best_solutions(solutions);
    let type_vars = close_transitively(type_vars)?;

    Ok((type_vars, world))
}

/// This function keeps only the solutions whose condition holds in the given world.
fn solutions_in_world(mut solutions: Vec<Solution>, world: &World) -> Vec<Solution> {
    for (choice, alternative) in world {
        solutions.retain(|s| match s.condition.get(choice) {
            Some(selected) => alternative == selected,

            // if the solution doesn't have a choice for the wanted name, it is invariant to the choice. So it is part of the world
            None => true,
        });
    }
    solutions
}

/// This function selects one solution for each type variable.
///
/// There could be several solutions for one type var. The best solution is chosen, which is the
/// solution with the least type vars in the ty. There could also be several solutions with
/// different conditions, but they are non conflicting, since this would be found by the
/// unification.
fn best_solutions(solutions: Vec<Solution>) -> TypeVarMapping {
    let mut type_vars = TypeVarMapping::new();

    for solution in solutions {
        if let Some(subst_ty) = type_vars.get_mut(&solution.var) {
            if solution.ty.type_var_names().len() < subst_ty.type_var_names().len() {
                *subst_ty = solution.ty;
            }
        } else {
            type_vars.insert(solution.var, solution.ty);
        }
    }

    type_vars
}

/// This function replaces the type variables in the solutions by their own solutions until only
/// type variables without a solution remain, i.e., it computes the transitive closure of the
/// mapping. A type variable that would have to contain itself has no finite type.
fn close_transitively(type_vars: TypeVarMapping) -> Result<TypeVarMapping, Error> {
    let mut closed = TypeVarMapping::new();
    for var in type_vars.keys() {
        close_type_var(var, &type_vars, &mut closed, &mut Vec::new())?;
    }
    Ok(closed)
}

/// This function computes the closed type of a single type variable, see
/// [`close_transitively`].
/// - `var` is the type variable.
/// - `type_vars` maps type variables to their solutions.
/// - `closed` stores the closed types computed so far.
/// - `visiting` are the type variables whose closed type is being computed, in the order of the
///   recursion. Reaching one of them again means that the solutions form a cycle.
fn close_type_var(
    var: &Name,
    type_vars: &TypeVarMapping,
    closed: &mut TypeVarMapping,
    visiting: &mut Vec<Name>,
) -> Result<Ty, Error> {
    if let Some(ty) = closed.get(var) {
        return Ok(ty.clone());
    }
    // a type variable without a solution stays as it is
    let Some(ty) = type_vars.get(var) else {
        return Ok(Ty::mk_ty_var(var));
    };

    if let Some(start) = visiting.iter().position(|visited| visited == var) {
        // a cycle only through type variables just says that these variables are equal, so they
        // stay without a solution; a cycle through a declared type would make the type infinite
        let declared_type_in_cycle = visiting[start..]
            .iter()
            .map(|visited| &type_vars[visited])
            .find(|solution| !matches!(solution, Ty::TypeVar { .. }));
        return match declared_type_in_cycle {
            None => Ok(Ty::mk_ty_var(var)),
            Some(infinite_ty) => Err(Error::InfiniteType {
                span: infinite_ty.span(),
                ty: infinite_ty.print_to_string(None),
            }),
        };
    }

    visiting.push(var.clone());
    let mut inner_types = TypeVarMapping::new();
    for inner_var in ty.type_var_names() {
        let inner_ty = close_type_var(&inner_var, type_vars, closed, visiting)?;
        inner_types.insert(inner_var, inner_ty);
    }
    visiting.pop();

    let mut closed_ty = ty.clone();
    closed_ty.mut_subst_ty(&inner_types);
    closed.insert(var.clone(), closed_ty.clone());
    Ok(closed_ty)
}

#[cfg(test)]
mod test {
    use crate::{
        syntax::{Ty, TypeArgs},
        typing::{
            Error,
            inference::{OverloadChoice, VariationalConstraint, solve_inference_constraints},
        },
    };

    /// This function creates the type `List[ty]`.
    fn list(ty: Ty) -> Ty {
        Ty::mk_decl("List", TypeArgs::mk(vec![ty]))
    }

    #[test]
    fn unconditional_conflict_with_choices() {
        let constraints = vec![
            VariationalConstraint::equal_if(Ty::mk_ty_var("x"), Ty::mk_i64(), 0, 0),
            VariationalConstraint::equal_if(Ty::mk_ty_var("x"), list(Ty::mk_i64()), 0, 1),
            VariationalConstraint::equal(Ty::mk_i64(), list(Ty::mk_i64())),
        ];
        let choices = [OverloadChoice {
            id: 0,
            alternatives: 2,
        }];

        let result = solve_inference_constraints(constraints, &choices);

        assert!(matches!(result, Err(Error::ConflictingTypes { .. })));
    }

    #[test]
    fn ambiguity_without_conflicts() {
        let constraints = vec![
            VariationalConstraint::equal_if(Ty::mk_ty_var("x"), Ty::mk_i64(), 0, 0),
            VariationalConstraint::equal_if(Ty::mk_ty_var("x"), Ty::mk_i64(), 0, 1),
        ];
        let choices = [OverloadChoice {
            id: 0,
            alternatives: 2,
        }];

        let result = solve_inference_constraints(constraints, &choices);

        assert!(matches!(
            result,
            Err(Error::AmbiguousOverloadResolution { number_worlds: 2 })
        ));
    }

    #[test]
    fn indirect_cycle_is_infinite() {
        let constraints = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("a"), list(Ty::mk_ty_var("b"))),
            VariationalConstraint::equal(Ty::mk_ty_var("b"), list(Ty::mk_ty_var("a"))),
        ];

        let result = solve_inference_constraints(constraints, &[]);

        assert!(matches!(result, Err(Error::InfiniteType { .. })));
    }

    #[test]
    fn variable_cycle_stays_unsolved() {
        let constraints = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("a"), Ty::mk_ty_var("b")),
            VariationalConstraint::equal(Ty::mk_ty_var("b"), Ty::mk_ty_var("a")),
        ];

        let (type_vars, _) = solve_inference_constraints(constraints, &[]).unwrap();

        assert!(matches!(type_vars["a"], Ty::TypeVar { .. }));
        assert!(matches!(type_vars["b"], Ty::TypeVar { .. }));
    }
}
