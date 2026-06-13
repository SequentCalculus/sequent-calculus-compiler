use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::syntax::{ID, Prog};

/// This trait defines a method for collecting quantity and flow constraints for steadfast inference.
pub trait CollectSteadfastConstraints {
    fn collect_sf_constraints(
        &self,
        program: &Prog,
        context: &[ID],
        constraints: &mut SteadfastConstraints,
    );
}

pub trait SubstituteSteadfastAnnotations {
    fn subst_sf_annotations(self, quantity_constraints: &HashSet<ID>) -> Self;
}

impl<T: SubstituteSteadfastAnnotations> SubstituteSteadfastAnnotations for Vec<T> {
    fn subst_sf_annotations(mut self, quantity_constraints: &HashSet<ID>) -> Self {
        let new = Vec::with_capacity(self.len());
        for mut x in std::mem::replace(&mut self, new) {
            x = x.subst_sf_annotations(quantity_constraints);
            self.push(x);
        }
        self
    }
}

impl<T: SubstituteSteadfastAnnotations + Clone> SubstituteSteadfastAnnotations for Rc<T> {
    fn subst_sf_annotations(self, quantity_constraints: &HashSet<ID>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).subst_sf_annotations(quantity_constraints))
    }
}

impl<T: SubstituteSteadfastAnnotations> SubstituteSteadfastAnnotations for Option<T> {
    fn subst_sf_annotations(self, quantity_constraints: &HashSet<ID>) -> Self {
        self.map(|x| x.subst_sf_annotations(quantity_constraints))
    }
}

#[derive(Debug, Default)]
pub struct SteadfastConstraints {
    pub quantity_constraints: HashSet<ID>,
    pub flow_constraints: HashMap<ID, HashSet<ID>>,
}

impl SteadfastConstraints {
    pub fn var_flows_into_var(&mut self, into: ID, from: ID) {
        self.flow_constraints.entry(into).or_default().insert(from);
    }

    pub fn vars_flow_together(&mut self, a: ID, b: ID) {
        self.var_flows_into_var(a, b);
        self.var_flows_into_var(b, a);
    }

    pub fn context_flows_into_var(&mut self, into: ID, context: &[ID]) {
        for from in context {
            self.var_flows_into_var(into, *from);
        }
    }

    pub fn contexts_flow_together(&mut self, a: &[ID], b: &[ID]) {
        debug_assert_eq!(a.len(), b.len());

        for (a, b) in a.iter().zip(b.iter()) {
            self.var_flows_into_var(*a, *b);
            self.var_flows_into_var(*b, *a);
        }
    }

    pub fn resolve(&self) -> HashSet<ID> {
        let mut solution = self.quantity_constraints.clone();
        let mut worklist: Vec<ID> = self.quantity_constraints.iter().copied().collect();

        while let Some(current) = worklist.pop() {
            if let Some(neighbors) = self.flow_constraints.get(&current) {
                for neighbor in neighbors {
                    if !solution.contains(neighbor) {
                        solution.insert(*neighbor);
                        worklist.push(*neighbor);
                    }
                }
            }
        }

        solution
    }
}
