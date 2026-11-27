use biodivine_lib_bdd::{Bdd, BddValuation, BddVariable, BddVariableSet, BddVariableSetBuilder};
use std::collections::HashMap;

use crate::typing::{
    Error,
    inference::{Alternative, ChoiceConflict, ChoiceId, OverloadChoice, World},
};

/// Since the choices are represented with several variables, the mapping between BDD Variables and Choice Variables is stored here.
/// In the first iteration, the choices are one-hot encoded
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ChoiceEncoding {
    /// Maps each choice variable to its bit variables (logarithmic encoding)
    alternative_vars: HashMap<ChoiceId, Vec<BddVariable>>,
    var_to_alternative: HashMap<BddVariable, (ChoiceId, Alternative)>,
}

impl ChoiceEncoding {
    fn register_choice(&mut self, choice: ChoiceId, bit_vars: Vec<BddVariable>) {
        self.alternative_vars.insert(choice, bit_vars.clone());
        for (idx, variable) in bit_vars.iter().enumerate() {
            self.var_to_alternative.insert(*variable, (choice, idx));
        }
    }

    pub fn alternative_vars_of(&self, choice: &ChoiceId) -> Option<&Vec<BddVariable>> {
        self.alternative_vars.get(choice)
    }
}

fn one_alternative_per_choice(
    choices: &Vec<OverloadChoice>,
) -> (Bdd, ChoiceEncoding, BddVariableSet) {
    let mut builder = BddVariableSetBuilder::new();
    let mut encoding = ChoiceEncoding::default();

    for choice in choices {
        // Register the bits variables needed to represent the choice
        let bit_vars: Vec<_> = (0..choice.alternatives)
            .map(|bit| builder.make_variable(&format!("{}_choice{}", choice.id, bit)))
            .collect();

        encoding.register_choice(choice.id, bit_vars);
    }

    let var_set = builder.build();

    let mut all_clauses = Vec::new();

    for choice in choices {
        let bdd_vars = encoding.alternative_vars_of(&choice.id).unwrap();

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

    (combined_clause, encoding, var_set)
}

fn exclude_conflicts(
    var_set: &BddVariableSet,
    encoding: &ChoiceEncoding,
    conflicts: &Vec<ChoiceConflict>,
) -> Bdd {
    let mut clauses = Vec::new();

    for conflict in conflicts {
        let mut clause_parts = Vec::new();

        for (choice, alternative) in conflict.condition.clone() {
            if let Some(bit_var) = encoding
                .alternative_vars_of(&choice)
                .unwrap_or_else(|| panic!("choice {} could not be found", choice))
                .get(alternative)
            {
                let bdd_var = var_set.mk_var(*bit_var);

                clause_parts.push(bdd_var);
            } else {
                panic!(
                    "Alternative bit {} for choice {} could not be found",
                    alternative, choice
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

fn decode_world(valuation: BddValuation, encoding: &ChoiceEncoding) -> World {
    valuation
        .to_values()
        .into_iter()
        .filter(|(_, truth_value)| *truth_value)
        .map(|(bdd_var, _)| {
            *encoding
                .var_to_alternative
                .get(&bdd_var)
                .expect("BDDVariable could not be found")
        })
        .collect()
}

pub fn select_world(
    choices: &Vec<OverloadChoice>,
    conflicts: Vec<ChoiceConflict>,
) -> Result<World, Error> {
    let (base_clauses, encoding, var_set) = one_alternative_per_choice(choices);

    let conflict_clauses = exclude_conflicts(&var_set, &encoding, &conflicts);

    let formula = base_clauses.and(&conflict_clauses);

    let possible_worlds = formula.cardinality();

    if possible_worlds > 1.0 {
        let mut worlds: Vec<World> = formula
            .sat_valuations()
            .map(|valuation| decode_world(valuation, &encoding))
            .collect();

        let first_world = worlds[0].clone();

        for (choice, alternative) in first_world {
            if worlds
                .iter()
                .all(|world| world.get(&choice) == Some(&alternative))
            {
                for world in worlds.iter_mut() {
                    world.remove(&choice);
                }
            }
        }

        return Err(Error::AmbiguousOverloadResolution {
            number_worlds: possible_worlds as u32,
        });
    } else if possible_worlds < 1.0 {
        return Err(Error::NoOverloadResolution {});
    }

    if let Some(valuation) = formula.sat_valuations().next() {
        Ok(decode_world(valuation, &encoding))
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
        inference::{ChoiceConflict, Condition, OverloadChoice, World},
        world_resolution::{
            ChoiceEncoding, exclude_conflicts, one_alternative_per_choice, select_world,
        },
    };
    use biodivine_lib_bdd::BddVariableSet;

    /// This function builds the choices for the given pairs of identifier and number of
    /// alternatives.
    fn choices(pairs: &[(u32, usize)]) -> Vec<OverloadChoice> {
        pairs
            .iter()
            .map(|&(id, alternatives)| OverloadChoice { id, alternatives })
            .collect()
    }

    #[test]
    fn base_clauses_test_single() {
        let choices = choices(&[(42, 3)]);
        let (resulting_clause, resulting_encoding, resulting_var_set) =
            one_alternative_per_choice(&choices);

        let expected_var_set = BddVariableSet::new(&["42_choice0", "42_choice1", "42_choice2"]);

        let mut expected_encoding = ChoiceEncoding::default();
        expected_encoding.register_choice(42, expected_var_set.variables());
        assert_eq!(expected_encoding, resulting_encoding);

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
        let choices = choices(&[(41, 3), (42, 4), (43, 2)]);
        let (resulting_clause, resulting_encoding, resulting_var_set) =
            one_alternative_per_choice(&choices);

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

        let mut expected_encoding = ChoiceEncoding::default();
        expected_encoding.register_choice(41, expected_var_set.variables()[0..3].to_vec());
        expected_encoding.register_choice(42, expected_var_set.variables()[3..7].to_vec());
        expected_encoding.register_choice(43, expected_var_set.variables()[7..9].to_vec());
        assert_eq!(expected_encoding, resulting_encoding);

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

    /// This function builds a conflict for the given selection of alternatives.
    fn mk_conflict(condition: Vec<(u32, usize)>) -> ChoiceConflict {
        ChoiceConflict {
            condition: Condition::from_iter(condition),
            error: crate::typing::Error::ConflictingTypes {
                span: None,
                lhs: "Test".to_string(),
                rhs: "Test".to_string(),
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

        let mut encoding = ChoiceEncoding::default();
        encoding.register_choice(41, var_set.variables()[0..3].to_vec());
        encoding.register_choice(42, var_set.variables()[3..7].to_vec());
        encoding.register_choice(43, var_set.variables()[7..9].to_vec());

        let conflicts: Vec<ChoiceConflict> = vec![
            mk_conflict(vec![(41, 2), (42, 1)]),
            mk_conflict(vec![(42, 0), (43, 0), (41, 1)]),
        ];

        let resulting_clauses = exclude_conflicts(&var_set, &encoding, &conflicts);

        let sub_clause_1 = var_set.eval_expression_string("!(41_choice2 & 42_choice1)");
        let sub_clause_2 =
            var_set.eval_expression_string("!(42_choice0 & 43_choice0 & 41_choice1)");

        let expected_clauses = sub_clause_1.and(&sub_clause_2);

        assert_eq!(resulting_clauses, expected_clauses);
    }

    #[test]
    fn select_world_test1() {
        let choices = choices(&[(41, 3), (42, 2)]);
        let conflicts: Vec<ChoiceConflict> = vec![
            mk_conflict(vec![(41, 2), (42, 1)]),
            mk_conflict(vec![(42, 0)]),
            mk_conflict(vec![(41, 1)]),
        ];

        let result = select_world(&choices, conflicts).unwrap();

        // there is only one correct solution
        let expected = World::from([(41, 0), (42, 1)]);

        assert_eq!(result, expected);
    }

    #[test]
    fn select_world_test2() {
        let choices = choices(&[(41, 3), (42, 4), (43, 2)]);
        let conflicts: Vec<ChoiceConflict> = vec![
            mk_conflict(vec![(41, 2), (42, 1)]),
            mk_conflict(vec![(42, 0)]),
            mk_conflict(vec![(41, 1)]),
            mk_conflict(vec![(43, 1)]),
            mk_conflict(vec![(41, 0), (42, 3), (43, 0)]),
        ];

        let result = select_world(&choices, conflicts);

        assert!(result.is_err());
    }
}
