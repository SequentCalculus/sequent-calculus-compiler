//! This module combines the phases of type inference that follow the collection of constraints:
//! unification, overload resolution and the construction of the final mapping of type variables.

use crate::typing::{
    Error,
    inference::{
        OverloadChoice, Solution, TypeVarMapping, VariationalConstraint, World, select_world,
        unify_variational,
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

    // if there are no choices, world resolving is skipped
    let world = if !choices.is_empty() {
        select_world(choices, conflicts)?
    } else if !conflicts.is_empty() {
        // there is only one world, but there are also conflicts. So there is no solution
        // todo!("Better Error")
        return Err(conflicts[0].error.clone());
    } else {
        World::new()
    };

    let solutions = solutions_in_world(solutions, &world);
    let type_vars = best_solutions(solutions);
    let type_vars = close_transitively(type_vars);

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

/// This function applies the mapping of type variables to itself until no type contains a type
/// variable anymore, i.e., it computes the transitive closure of the mapping.
fn close_transitively(mut type_vars: TypeVarMapping) -> TypeVarMapping {
    let reference_mapping = type_vars.clone();

    for ty in type_vars.values_mut() {
        loop {
            let var_names = ty.type_var_names();

            if var_names.is_empty() {
                break;
            }

            if var_names.iter().any(|s| !reference_mapping.contains_key(s)) {
                let missing_names: Vec<String> = ty
                    .type_var_names()
                    .into_iter()
                    .filter(|k| !reference_mapping.contains_key(k))
                    .collect();
                panic!(
                    "Missing type var names in the final type mapping: {:?}",
                    missing_names
                );
            }

            ty.mut_subst_ty(&reference_mapping);
        }
    }

    type_vars
}
