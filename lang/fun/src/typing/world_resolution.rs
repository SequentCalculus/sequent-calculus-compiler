use biodivine_lib_bdd::{Bdd, BddValuation, BddVariable, BddVariableSet, BddVariableSetBuilder};
use std::collections::HashMap;

use crate::typing::{Error, inference::IncompatibleChoices};

type PossibleChoice = (u32, usize);

/// Since the choices are represented with several variables, the mapping between BDD Variables and Choice Variables is stored here.
/// In the first iteration, the choices are one-hot encoded
#[derive(Debug, PartialEq, Eq, Default)]
pub struct BddMapping {
    /// Maps each choice variable to its bit variables (logarithmic encoding)
    choice_bit_vars: HashMap<u32, Vec<BddVariable>>,
    variable_resolving: HashMap<BddVariable, (u32, usize)>,
}

impl BddMapping {
    fn add_choice(&mut self, choice_id: u32, bit_vars: Vec<BddVariable>) {
        self.choice_bit_vars.insert(choice_id, bit_vars.clone());
        for (idx, variable) in bit_vars.iter().enumerate() {
            self.variable_resolving.insert(*variable, (choice_id, idx));
        }
    }

    pub fn get_bit_vars(&self, choice_var: &u32) -> Option<&Vec<BddVariable>> {
        self.choice_bit_vars.get(choice_var)
    }
}

fn create_base_bdd(choices: &Vec<PossibleChoice>) -> (Bdd, BddMapping, BddVariableSet) {
    let mut builder = BddVariableSetBuilder::new();
    let mut mapping = BddMapping::default();

    for (cvar, num_alternatives) in choices {
        // Register the bits variables needed to represent the choice
        let bit_vars: Vec<_> = (0..*num_alternatives)
            .map(|bit| builder.make_variable(&format!("{}_choice{}", cvar, bit)))
            .collect();

        mapping.add_choice(*cvar, bit_vars);
    }

    let var_set = builder.build();

    let mut all_clauses = Vec::new();

    for (cvar, _) in choices {
        let bdd_vars = mapping.get_bit_vars(cvar).unwrap();

        let bdd_var_terms: Vec<Bdd> = bdd_vars.iter().map(|v| var_set.mk_var(*v)).collect();

        let mut choice_clauses = Vec::new();

        for true_index in 0..bdd_var_terms.len() {
            let mut term = var_set.mk_true();

            for (var_index, var) in bdd_var_terms.iter().enumerate() {
                if var_index == true_index {
                    term = term.and(var);
                } else {
                    term = term.and_not(var);
                }
            }

            choice_clauses.push(term);
        }

        all_clauses.push(
            choice_clauses
                .iter()
                .fold(var_set.mk_false(), |acc, clause| acc.or(clause)),
        );
    }

    if all_clauses.is_empty() {
        panic!("No base clauses were created")
    }

    let combined_clause = all_clauses
        .iter()
        .fold(var_set.mk_true(), |acc, next| acc.and(next));

    (combined_clause, mapping, var_set)
}

fn create_fail_clauses(
    var_set: &BddVariableSet,
    mapping: &BddMapping,
    incompatible_worlds: &Vec<IncompatibleChoices>,
) -> Bdd {
    let mut clauses = Vec::new();

    for world in incompatible_worlds {
        let mut clause_parts = Vec::new();

        for (choice_id, bit_idx) in world.choices.clone() {
            if let Some(bit_var) = mapping
                .get_bit_vars(&choice_id)
                .unwrap_or_else(|| panic!("choice {} could not be found", choice_id))
                .get(bit_idx)
            {
                let bdd_var = var_set.mk_var(*bit_var);

                clause_parts.push(bdd_var);
            } else {
                panic!(
                    "Alternative bit {} for choice {} could not be found",
                    bit_idx, choice_id
                )
            }
        }
        if !clause_parts.is_empty() {
            let completed_clause = clause_parts
                .iter()
                .skip(1)
                .fold(clause_parts[0].clone(), |acc, next| acc.and(next))
                .not();

            clauses.push(completed_clause);
        }
    }

    clauses
        .iter()
        .skip(1)
        .fold(clauses[0].clone(), |acc, next| acc.and(next))
}

fn bddvaluation_2_choices(solution: BddValuation, mapping: &BddMapping) -> Vec<(u32, usize)> {
    solution
        .to_values()
        .into_iter()
        .filter(|(_, truth_value)| *truth_value)
        .map(|(bdd_var, _)| {
            *mapping
                .variable_resolving
                .get(&bdd_var)
                .expect("BDDVariable could not be found")
        })
        .collect()
}

pub fn resolve_worlds(
    choices: &Vec<PossibleChoice>,
    incompatible_choices: Vec<IncompatibleChoices>,
) -> Result<Vec<PossibleChoice>, Error> {
    let (base_clauses, mapping, var_set) = create_base_bdd(choices);

    let incompatible_clauses = create_fail_clauses(&var_set, &mapping, &incompatible_choices);

    let combined_formular = base_clauses.and(&incompatible_clauses);

    let possible_worlds = combined_formular.cardinality();

    if possible_worlds > 1.0 {
        let possible_choices: Vec<Vec<(u32, usize)>> = combined_formular
            .sat_valuations()
            .map(|valuation| bddvaluation_2_choices(valuation, &mapping))
            .collect();

        let mut choice_maps: Vec<HashMap<u32, usize>> = possible_choices
            .into_iter()
            .map(HashMap::from_iter)
            .collect();

        let first_map = choice_maps[0].clone();

        for (choice_id, signature_idx) in first_map {
            if choice_maps
                .iter()
                .all(|mapping| mapping.get(&choice_id) == Some(&signature_idx))
            {
                for mapping in choice_maps.iter_mut() {
                    mapping.remove(&choice_id);
                }
            }
        }

        return Err(Error::MoreThanOneWorld {
            number_worlds: possible_worlds as u32,
        });
    } else if possible_worlds < 1.0 {
        return Err(Error::NoPossibleoWorld {});
    }

    if let Some(solution) = combined_formular.sat_valuations().next() {
        let selected_choices = bddvaluation_2_choices(solution, &mapping);

        Ok(selected_choices)
    } else {
        panic!(
            "No Solution found although there were {} solutions calculated",
            possible_worlds
        )
    }
}

#[cfg(test)]
mod test {
    use crate::typing::{
        inference::IncompatibleChoices,
        world_resolution::{
            BddMapping, PossibleChoice, create_base_bdd, create_fail_clauses, resolve_worlds,
        },
    };
    use biodivine_lib_bdd::BddVariableSet;

    #[test]
    fn base_clauses_test_single() {
        let possible_choices: Vec<(u32, usize)> = vec![(42, 3)];
        let (resulting_clause, resulting_mapping, resulting_var_set) =
            create_base_bdd(&possible_choices);

        let expected_var_set = BddVariableSet::new(&["42_choice0", "42_choice1", "42_choice2"]);

        let mut expected_mapping = BddMapping::default();
        expected_mapping.add_choice(42, expected_var_set.variables());
        assert_eq!(expected_mapping, resulting_mapping);

        // the resulting var set is used, because two different var sets can make problems if you compare the terms
        let sub_clause_1 =
            resulting_var_set.eval_expression_string("42_choice0 | 42_choice1 | 42_choice2");
        let sub_clause_2 = resulting_var_set.eval_expression_string("!(42_choice0 & 42_choice1)");
        let sub_clause_3 = resulting_var_set.eval_expression_string("!(42_choice0 & 42_choice2)");
        let sub_clause_4 = resulting_var_set.eval_expression_string("!(42_choice1 & 42_choice2)");

        let all_clauses = sub_clause_1
            .and(&sub_clause_2)
            .and(&sub_clause_3)
            .and(&sub_clause_4);

        assert_eq!(all_clauses, resulting_clause);
    }

    #[test]
    fn base_clauses_test_multi() {
        let possible_choices: Vec<PossibleChoice> = vec![(41, 3), (42, 4), (43, 2)];
        let (resulting_clause, resulting_mapping, resulting_var_set) =
            create_base_bdd(&possible_choices);

        let expected_var_set = BddVariableSet::new(&[
            "41_choice0",
            "41_choice1",
            "41_choice2",
            "42_choice0",
            "42_choice1",
            "42_choice2",
            "42_choice3",
            "43_choice0",
            "43_choice1",
        ]);
        assert_eq!(expected_var_set.variables(), resulting_var_set.variables());

        let mut expected_mapping = BddMapping::default();
        expected_mapping.add_choice(41, expected_var_set.variables()[0..3].to_vec());
        expected_mapping.add_choice(42, expected_var_set.variables()[3..7].to_vec());
        expected_mapping.add_choice(43, expected_var_set.variables()[7..9].to_vec());
        assert_eq!(expected_mapping, resulting_mapping);

        // the resulting var set is used, because two different var sets can make problems if you compare the terms

        // -- add
        let sub_clause_1 =
            resulting_var_set.eval_expression_string("41_choice0 | 41_choice1 | 41_choice2");
        let sub_clause_2 = resulting_var_set.eval_expression_string("!(41_choice0 & 41_choice1)");
        let sub_clause_3 = resulting_var_set.eval_expression_string("!(41_choice0 & 41_choice2)");
        let sub_clause_4 = resulting_var_set.eval_expression_string("!(41_choice1 & 41_choice2)");

        // -- new
        let sub_clause_5 = resulting_var_set
            .eval_expression_string("42_choice0 | 42_choice1 | 42_choice2 | 42_choice3");
        let sub_clause_6 = resulting_var_set.eval_expression_string("!(42_choice0 & 42_choice1)");
        let sub_clause_7 = resulting_var_set.eval_expression_string("!(42_choice0 & 42_choice2)");
        let sub_clause_8 = resulting_var_set.eval_expression_string("!(42_choice0 & 42_choice3)");
        let sub_clause_9 = resulting_var_set.eval_expression_string("!(42_choice1 & 42_choice2)");
        let sub_clause_10 = resulting_var_set.eval_expression_string("!(42_choice1 & 42_choice3)");
        let sub_clause_11 = resulting_var_set.eval_expression_string("!(42_choice2 & 42_choice3)");

        // -- func
        let sub_clause_12 = resulting_var_set.eval_expression_string("43_choice0 | 43_choice1");
        let sub_clause_13 = resulting_var_set.eval_expression_string("!(43_choice0 & 43_choice1)");

        let all_clauses = sub_clause_1
            .and(&sub_clause_2)
            .and(&sub_clause_3)
            .and(&sub_clause_4)
            .and(&sub_clause_5)
            .and(&sub_clause_6)
            .and(&sub_clause_7)
            .and(&sub_clause_8)
            .and(&sub_clause_9)
            .and(&sub_clause_10)
            .and(&sub_clause_11)
            .and(&sub_clause_12)
            .and(&sub_clause_13);

        assert_eq!(all_clauses, resulting_clause);
    }

    fn mk_inc_choice(choices: Vec<(u32, usize)>) -> IncompatibleChoices {
        IncompatibleChoices {
            choices,
            error: crate::typing::Error::ConflictingTypeConstraints {
                span_l: None,
                expected_type_l: "Test".to_string(),
                expected_type_r: "Test".to_string(),
            },
        }
    }

    #[test]
    fn fail_clause_test1() {
        let var_set = BddVariableSet::new(&[
            "41_choice0",
            "41_choice1",
            "41_choice2",
            "42_choice0",
            "42_choice1",
            "42_choice2",
            "42_choice3",
            "43_choice0",
            "43_choice1",
        ]);

        let mut mapping = BddMapping::default();
        mapping.add_choice(41, var_set.variables()[0..3].to_vec());
        mapping.add_choice(42, var_set.variables()[3..7].to_vec());
        mapping.add_choice(43, var_set.variables()[7..9].to_vec());

        let incompatible_choices: Vec<IncompatibleChoices> = vec![
            mk_inc_choice(vec![(41, 2), (42, 1)]),
            mk_inc_choice(vec![(42, 0), (43, 0), (41, 1)]),
        ];

        let resulting_clauses = create_fail_clauses(&var_set, &mapping, &incompatible_choices);

        let sub_clause_1 = var_set.eval_expression_string("!(41_choice2 & 42_choice1)");
        let sub_clause_2 =
            var_set.eval_expression_string("!(42_choice0 & 43_choice0 & 41_choice1)");

        let expected_clauses = sub_clause_1.and(&sub_clause_2);

        assert_eq!(resulting_clauses, expected_clauses);
    }

    #[test]
    fn resolve_worlds_test1() {
        let possible_choices: Vec<PossibleChoice> = vec![(41, 3), (42, 2)];
        let incompatible_choices: Vec<IncompatibleChoices> = vec![
            mk_inc_choice(vec![(41, 2), (42, 1)]),
            mk_inc_choice(vec![(42, 0)]),
            mk_inc_choice(vec![(41, 1)]),
        ];

        let result = resolve_worlds(&possible_choices, incompatible_choices).unwrap();

        // there is only one correct solution
        let expected = vec![(41, 0), (42, 1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn resolve_worlds_test2() {
        let possible_choices: Vec<PossibleChoice> = vec![(41, 3), (42, 4), (43, 2)];
        let incompatible_choices: Vec<IncompatibleChoices> = vec![
            mk_inc_choice(vec![(41, 2), (42, 1)]),
            mk_inc_choice(vec![(42, 0)]),
            mk_inc_choice(vec![(41, 1)]),
            mk_inc_choice(vec![(43, 1)]),
            mk_inc_choice(vec![(41, 0), (42, 3), (43, 0)]),
        ];

        let result = resolve_worlds(&possible_choices, incompatible_choices);

        assert!(result.is_err());
    }
}
