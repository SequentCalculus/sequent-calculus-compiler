//! A constraint solver for type equality constraints.

use std::{collections::HashMap, rc::Rc};

use crate::syntax::Variable;

use super::{Ty, Typevar, Zonk};

pub type Constraint = (Ty, Ty);

impl Zonk for Constraint {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Constraint {
        (Zonk::zonk(&self.0, varmap), Zonk::zonk(&self.1, varmap))
    }
}

type Error = String;

struct SolverState {
    todo: Vec<Constraint>,
    subst: HashMap<Typevar, Ty>,
}

impl SolverState {
    fn add_constraints(&mut self, new_constraints: Vec<Constraint>) {
        self.todo.extend(new_constraints);
    }
}

pub fn solve_constraints(constraints: Vec<Constraint>) -> Result<HashMap<Typevar, Ty>, Error> {
    let mut initial = SolverState {
        todo: constraints,
        subst: HashMap::new(),
    };
    run(&mut initial)?;
    Ok(initial.subst)
}

fn perform_subst(var: Typevar, ty: Ty, st: &mut SolverState) {
    let subst: HashMap<Variable, Ty> = HashMap::from([(var, ty)]);
    let new_todo: Vec<Constraint> = st
        .todo
        .iter()
        .map(|constraint| Zonk::zonk(constraint, &subst))
        .collect();
    let mut new_subst: HashMap<String, Ty> = Zonk::zonk(&st.subst, &subst);
    new_subst.extend(subst);
    st.subst = new_subst;
    st.todo = new_todo;
}

fn run(st: &mut SolverState) -> Result<(), Error> {
    while let Some(next_constraint) = st.todo.pop() {
        solve_constraint(next_constraint, st)?;
    }
    Ok(())
}

fn solve_constraint(constraint: Constraint, st: &mut SolverState) -> Result<(), Error> {
    match constraint {
        (Ty::Tyvar(a), Ty::Tyvar(b)) if a == b => Ok(()),
        (Ty::Tyvar(a), ty) => {
            if ty.free_tyvars().contains(&a) {
                Err(format!("Occurs check! {} occurs in {}", a, ty))
            } else {
                perform_subst(a, ty, st);
                Ok(())
            }
        }
        (ty, Ty::Tyvar(a)) => {
            if ty.free_tyvars().contains(&a) {
                Err(format!("Occurs check! {} occurs in {}", a, ty))
            } else {
                perform_subst(a, ty, st);
                Ok(())
            }
        }
        (Ty::Int(), Ty::Int()) => Ok(()),
        (Ty::List(ty1), Ty::List(ty2)) => {
            st.add_constraints(vec![(Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty2))]);
            Ok(())
        }
        (Ty::Pair(ty1, ty2), Ty::Pair(ty3, ty4)) => {
            st.add_constraints(vec![
                (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
            ]);
            Ok(())
        }
        (Ty::Stream(ty1), Ty::Stream(ty2)) => {
            st.add_constraints(vec![(Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty2))]);
            Ok(())
        }
        (Ty::LPair(ty1, ty2), Ty::LPair(ty3, ty4)) => {
            st.add_constraints(vec![
                (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
            ]);
            Ok(())
        }
        (Ty::Fun(ty1, ty2), Ty::Fun(ty3, ty4)) => {
            st.add_constraints(vec![
                (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
            ]);
            Ok(())
        }
        (ty1, ty2) => Err(format!("Cannot unify types: {} and {}", ty1, ty2)),
    }
}
