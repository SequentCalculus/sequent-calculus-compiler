use std::{collections::HashMap, vec};
use biodivine_lib_bdd::{
    Bdd, BddPartialValuation, BddValuation, BddVariable, BddVariableSet, BddVariableSetBuilder,
};

use crate::{syntax::Name, typing::{Error, inference::IncompatibleChoices}};



type PossibleChoice = (Name, usize);

/// Since the choices are represented with several variables, the mapping between BDD Variables and Choice Variables is stored here.
/// In the first iteration, the choices are one-hot encoded
#[derive(Debug)]
pub struct BddMapping {
    /// Maps each choice variable to its bit variables (logarithmic encoding)
    choice_bit_vars: HashMap<Name, Vec<BddVariable>>,
    variable_resolving: HashMap<BddVariable, (Name, usize)>,
    /// Number of alternatives for each choice variable (for validation)
    choice_sizes: HashMap<Name, usize>,
    /// Ordered list of choice variables (for consistent solution ordering)
    choice_order: Vec<Name>,
}


impl BddMapping {
    fn new() -> Self {
        Self {
            choice_bit_vars: HashMap::default(),
            variable_resolving: HashMap::default(),
            choice_sizes: HashMap::default(),
            choice_order: Vec::new(),
        }
    }

    fn add_choice(
        &mut self,
        choice_var: Name,
        bit_vars: Vec<BddVariable>,
        num_alternatives: usize,
    ) {
        self.choice_bit_vars.insert(choice_var.clone(), bit_vars.clone());
        for (idx, variable) in bit_vars.iter().enumerate() {
            self.variable_resolving.insert(*variable, (choice_var.clone(), idx));
        }
        self.choice_sizes
            .insert(choice_var.clone(), num_alternatives);
        self.choice_order.push(choice_var.clone());
        
    }

    pub fn get_bit_vars(&self, choice_var: &Name) -> Option<&Vec<BddVariable>> {
        self.choice_bit_vars.get(choice_var)
    }

    pub fn all_choice_vars(&self) -> impl Iterator<Item = &Name> {
        self.choice_bit_vars.keys()
    }
}

fn create_base_bdd(choices: &Vec<PossibleChoice>) -> (Bdd, BddMapping, BddVariableSet) {
    let mut builder = BddVariableSetBuilder::new();
    let mut mapping = BddMapping::new();

    for (cvar, num_alternatives) in choices {
        // Register the bits variables needed to represent the choice
        let bit_vars: Vec<_> = (0..*num_alternatives)
            .map(|bit| builder.make_variable(&format!("{}_choice{}", cvar, bit)))
            .collect();

        mapping.add_choice(cvar.clone(), bit_vars, *num_alternatives);
    }

    let var_set = builder.build();

    let mut clauses = Vec::new();

    
    for (cvar, _) in choices {
        let bdd_vars = mapping.get_bit_vars(cvar).unwrap();

        let bdd_terms: Vec<Bdd> = bdd_vars.iter().map(|v| var_set.mk_var(*v)).collect();

        let at_least_once_term = bdd_terms.iter().skip(1).fold(bdd_terms[0].clone(), |acc, next| acc.or(next));
        clauses.push(at_least_once_term);


        let mut at_most_terms = Vec::new();

        for (var_idx, var_term_a) in bdd_terms.iter().enumerate() {
            for var_term_b in bdd_terms.iter().skip(var_idx) {
                at_most_terms.push(var_term_a.and(var_term_b).not());
            }
        }

        let combined_at_most_once_term = at_most_terms.iter().skip(1).fold(at_most_terms[0].clone(), |acc, next| acc.and(next));

        clauses.push(combined_at_most_once_term);
        
    }

    if clauses.is_empty() {
        panic!("No base clauses were created")
    }

    let combined_clause = clauses.iter().skip(1).fold(clauses[0].clone(), |acc, next| acc.and(next));

    (combined_clause, mapping, var_set)
}


fn create_fail_clauses(var_set: &BddVariableSet, mapping: &BddMapping, incompatible_choices: Vec<IncompatibleChoices>) -> Bdd {

    let mut clauses = Vec::new();

    for choice in incompatible_choices {
        let mut clause_parts = Vec::new();

        for (var_name, bit_idx) in choice {
            if let Some(bit_var) = mapping.get_bit_vars(&var_name).expect(&format!("choice {} could not be found", var_name)).get(bit_idx) {
                let bdd_var = var_set.mk_var(*bit_var);

                clause_parts.push(bdd_var);
            } else {
                panic!("Alternative bit {} for choice {} could not be found", bit_idx, var_name)
            }
        }

        let completed_clause = clause_parts.iter().skip(1).fold(clause_parts[0].clone(), |acc, next| acc.and(next)).not();

        clauses.push(completed_clause);
    }

    clauses.iter().skip(1).fold(clauses[0].clone(), |acc, next| acc.and(next))
}


pub fn resolve_worlds(choices: &Vec<PossibleChoice>, incompatible_choices: Vec<IncompatibleChoices>) -> Result<Vec<(Name, usize)>, Error> {
    let (base_clauses, mapping, var_set) = create_base_bdd(choices);

    let incompatible_clauses = create_fail_clauses(&var_set, &mapping, incompatible_choices);

    let combined_formular = base_clauses.and(&incompatible_clauses);

    let possible_worlds = combined_formular.cardinality();

    if possible_worlds != 1.0 {
        return Err(Error::NotExactlyOneWorld { number_worlds: possible_worlds });
    }

    if let Some(solution) = combined_formular.sat_valuations().next() {
        let selected_variables: Vec<(BddVariable, bool)> = solution.to_values().into_iter().filter(|(_, truth_value)| *truth_value).collect();

        let selected_choices = selected_variables.iter().map(|(bdd_var, _)| mapping.variable_resolving.get(bdd_var).expect("BDDVariable could not be found").clone()).collect();

        Ok(selected_choices)
    } else {
        panic!("No Solution found although there were {} solutions calculated", possible_worlds)
    }
}