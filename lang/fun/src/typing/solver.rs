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

    fn run(&mut self) -> Result<(), Error> {
        while let Some(next_constraint) = self.todo.pop() {
            self.solve_constraint(next_constraint)?;
        }
        Ok(())
    }

    fn solve_constraint(&mut self, constraint: Constraint) -> Result<(), Error> {
        match constraint {
            (Ty::Var(a), Ty::Var(b)) if a == b => Ok(()),
            (Ty::Var(a), ty) => {
                if ty.free_tyvars().contains(&a) {
                    Err(format!("Occurs check! {} occurs in {}", a, ty))
                } else {
                    self.perform_subst(a, ty);
                    Ok(())
                }
            }
            (ty, Ty::Var(a)) => {
                if ty.free_tyvars().contains(&a) {
                    Err(format!("Occurs check! {} occurs in {}", a, ty))
                } else {
                    self.perform_subst(a, ty);
                    Ok(())
                }
            }
            (Ty::Int(), Ty::Int()) => Ok(()),
            (Ty::List(ty1), Ty::List(ty2)) => {
                self.add_constraints(vec![(Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty2))]);
                Ok(())
            }
            (Ty::Pair(ty1, ty2), Ty::Pair(ty3, ty4)) => {
                self.add_constraints(vec![
                    (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                    (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
                ]);
                Ok(())
            }
            (Ty::Stream(ty1), Ty::Stream(ty2)) => {
                self.add_constraints(vec![(Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty2))]);
                Ok(())
            }
            (Ty::LPair(ty1, ty2), Ty::LPair(ty3, ty4)) => {
                self.add_constraints(vec![
                    (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                    (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
                ]);
                Ok(())
            }
            (Ty::Fun(ty1, ty2), Ty::Fun(ty3, ty4)) => {
                self.add_constraints(vec![
                    (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                    (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
                ]);
                Ok(())
            }
            (ty1, ty2) => Err(format!("Cannot unify types: {} and {}", ty1, ty2)),
        }
    }

    fn perform_subst(&mut self, var: Typevar, ty: Ty) {
        let subst: HashMap<Variable, Ty> = HashMap::from([(var, ty)]);
        let new_todo: Vec<Constraint> = self
            .todo
            .iter()
            .map(|constraint| Zonk::zonk(constraint, &subst))
            .collect();
        let mut new_subst: HashMap<String, Ty> = Zonk::zonk(&self.subst, &subst);
        new_subst.extend(subst);
        self.subst = new_subst;
        self.todo = new_todo;
    }
}

pub fn solve_constraints(constraints: Vec<Constraint>) -> Result<HashMap<Typevar, Ty>, Error> {
    let mut initial = SolverState {
        todo: constraints,
        subst: HashMap::new(),
    };
    initial.run()?;
    Ok(initial.subst)
}

#[cfg(test)]
mod solver_tests {
    use std::{collections::HashMap, rc::Rc};

    use crate::typing::{Ty, Typevar};

    use super::solve_constraints;

    #[test]
    fn solve_empty() {
        let result = solve_constraints(vec![]);
        assert!(result.is_ok())
    }

    #[test]
    fn solve_int_int() {
        let result = solve_constraints(vec![(Ty::Int(), Ty::Int())]);
        assert!(result.is_ok())
    }

    #[test]
    fn solve_int_a() {
        let result = solve_constraints(vec![(Ty::Int(), Ty::Var("a".to_string()))]);
        let mut expected: HashMap<Typevar, Ty> = HashMap::new();
        expected.insert("a".to_string(), Ty::Int());
        assert_eq!(result, Ok(expected))
    }

    #[test]
    fn solve_a_int() {
        let result = solve_constraints(vec![(Ty::Var("a".to_string()), Ty::Int())]);
        let mut expected: HashMap<Typevar, Ty> = HashMap::new();
        expected.insert("a".to_string(), Ty::Int());
        assert_eq!(result, Ok(expected))
    }

    #[test]
    fn solve_int_list() {
        let result = solve_constraints(vec![(Ty::Int(), Ty::List(Rc::new(Ty::Int())))]);
        assert!(result.is_err())
    }

    #[test]
    fn solve_occurs_check() {
        // The constraint "a = List(a)" should result in an occurs-check failure.
        let result = solve_constraints(vec![(
            Ty::Var("a".to_string()),
            Ty::List(Rc::new(Ty::Var("a".to_string()))),
        )]);
        assert!(result.is_err())
    }

    #[test]
    fn solve_occurs_check_complex() {
        // The constraints "a = b" and "a = List(b)" should result in an occurs-check failure.
        let c1 = (Ty::Var("a".to_string()), Ty::Var("b".to_string()));
        let c2 = (Ty::Var("a".to_string()), Ty::List(Rc::new(Ty::Var("b".to_string()))));
        let result = solve_constraints(vec![c1,c2]);
        assert!(result.is_err())
    }
}
