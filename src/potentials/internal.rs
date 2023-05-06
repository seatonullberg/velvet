use itertools::Itertools;

use crate::errors::PotentialsInitializationError;
use crate::potentials::pair::group::MixingStrategy;
use crate::system::AtomType;

// Check that the given and required parameter names match.
pub fn check_parameter_names(
    given_names: &Vec<String>,
    required_names: &Vec<String>,
) -> Result<(), PotentialsInitializationError> {
    // Preemptively create an error to return.
    let err = PotentialsInitializationError::IncompatibleParameters {
        expected: required_names.clone(),
        found: given_names.clone(),
    };
    // Check that lengths match.
    if given_names.len() != required_names.len() {
        return Err(err);
    }
    // Check that all elements of `given_names` are in `required_names`.
    // Order does not matter.
    for given in given_names {
        if !required_names.contains(given) {
            return Err(err);
        }
    }
    Ok(())
}

// Check that the mixing strategy is compatible with other options for a pair potential group.
// pub fn check_mixing_strategy_compatibility<T>(
//     atom_types: &[(AtomType, AtomType)],
//     mixing_strategy: MixingStrategy,
//     links: Option<T>,
// ) -> Result<(), PotentialsInitializationError> {
//     // Preemptively define an error message variable that can be overwritten.
//     //let mut err_msg: Option<String> = Option::None;
//     // Match on the mixing strategy to determine possible incompatibilities.
//     let err_msg = match mixing_strategy {
//         // If explicit is being used, make sure that the list of atom types is exhaustive.
//         MixingStrategy::Explicit => {
//             if is_exhaustive(atom_types) {
//                 // The list is exhaustive so `Explicit` is ok.
//                 None
//             } else {
//                 // The list is not exhaustive so `Explicit` will not work.
//                 Some("atom type pairs must be exhaustive for this mixing strategy".to_string())
//             }
//         },
//         // If another strategy is being used, make sure that there are no links.
//         // Links require the use of the explicit mixing strategy.
//         _ => {
//             links.map(
//                 |_| "pair potential groups with links are required to use the `Explicit` mixing strategy".to_string()
//             )
//         },
//     };
//     // Return the error if one was detected or () if nothing is wrong.
//     err_msg.map_or_else(
//         || Ok(()),
//         |msg| {
//             let strategy = mixing_strategy;
//             let err = PotentialsInitializationError::InvalidMixingStrategy { strategy, msg };
//             Err(err)
//         },
//     )
// }

// Returns true when the list of atom type pairs is exhaustive.
// Example - for 3 atom types with IDs [1, 2, 3] the exhaustive
// list of combinations would be:
// [(1, 1), (1, 2), (1, 3), (2, 2), (2, 3), (3, 3)]
// This list can be unordered and the function will still work because it
// orders the inputs internally.
//
// There is probably a much more efficient way to do this but it works for now.
// fn is_exhaustive(pairs: &[(AtomType, AtomType)]) -> bool {
//     // First create a flattened vec of all atom types.
//     let mut atom_types: Vec<AtomType> = Vec::new();
//     for (i, j) in pairs {
//         atom_types.push(*i);
//         atom_types.push(*j);
//     }
//     // Sort the vec by each atom type's id.
//     atom_types.sort_by(|a, b| a.partial_cmp(&b).unwrap());
//     // Deduplicate the vec.
//     atom_types.dedup();
//     // Store the min and max IDs.
//     let minimum = atom_types[0].id();
//     let maximum = atom_types[atom_types.len() - 1].id();
//     // Check that the atom types are contiguous.
//     let range = maximum - minimum;
//     println!("{:?}", atom_types);
//     true
// }

#[cfg(test)]
mod tests {
    use crate::errors::PotentialsInitializationError;

    use super::check_parameter_names;

    // Check that the `check_parameter_names` function works for valid inputs.
    #[test]
    fn check_parameter_names_valid_inputs() {
        let given_names: Vec<String> = vec!["test2".to_string(), "test1".to_string()];
        let required_names: Vec<String> = vec!["test1".to_string(), "test2".to_string()];
        check_parameter_names(&given_names, &required_names).unwrap()
    }

    // Check that the `check_parameter_names` function returns the correct error when lengths are mismatched.
    #[test]
    fn check_parameter_names_returns_incompatible_parameters_error_for_length() {
        let given_names: Vec<String> = vec!["test1".to_string()];
        let required_names: Vec<String> = vec!["test1".to_string(), "test2".to_string()];
        match check_parameter_names(&given_names, &required_names) {
            Ok(_) => panic!("unexpected ok result"),
            Err(err) => match err {
                PotentialsInitializationError::IncompatibleParameters {
                    expected: _,
                    found: _,
                } => {}
                _ => panic!("unexpected error type"),
            },
        }
    }

    // Check that the `check_parameter_names` function returns the correct error when names are mismatched.
    #[test]
    fn check_parameter_names_returns_incompatible_parameters_error_for_names() {
        let given_names: Vec<String> = vec!["test1".to_string(), "2test".to_string()];
        let required_names: Vec<String> = vec!["test1".to_string(), "test2".to_string()];
        match check_parameter_names(&given_names, &required_names) {
            Ok(_) => panic!("unexpected ok result"),
            Err(err) => match err {
                PotentialsInitializationError::IncompatibleParameters {
                    expected: _,
                    found: _,
                } => {}
                _ => panic!("unexpected error type"),
            },
        }
    }

    // Check that the `check_mixing_strategy_compatibility` function works with valid exhaustive inputs.
    // #[test]
    // fn check_mixing_strategy_compatibility_valid_exhaustive_inputs() {
    //     let at1 = AtomType::new(0, 0.0, 0.0);
    //     let at2 = AtomType::new(1, 0.0, 0.0);
    //     let at3 = AtomType::new(2, 0.0, 0.0);
    //     let mut atom_types = vec![
    //         (at1, at3),
    //         (at1, at1),
    //         (at1, at2),
    //         (at2, at2),
    //         (at3, at3),
    //         (at2, at3),
    //     ];
    //     let strategy = MixingStrategy::Explicit;
    //     // usize will not be the option type in practice, but within the scope of this test it can be anything.
    //     // Usually the explicit type will be used with links but it is not necessary to.
    //     let links: Option<usize> = None;
    //     // The check should pass with no errors.
    //     check_mixing_strategy_compatibility(&mut atom_types, strategy, links).unwrap()
    // }

    // Check that the `check_mixing_strategy_compatibility` function returns the correct error when the explicit
    // mixing strategy is used but the atom type pairs are not exhaustive.
    // #[test]
    // fn check_mixing_strategy_compatibility_returns_invalid_mixing_strategy_error_explicit_non_exhaustive() {
    //     let at1 = AtomType::new(0, 0.0, 0.0);
    //     let at2 = AtomType::new(1, 0.0, 0.0);
    //     let at3 = AtomType::new(2, 0.0, 0.0);
    //     let mut atom_types = vec![
    //         (at1, at1),
    //         (at2, at2),
    //         (at3, at3),
    //     ];
    //     let strategy = MixingStrategy::Explicit;
    //     // usize will not be the option type in practice, but within the scope of this test it can be anything.
    //     // Usually the explicit type will be used with links but it is not necessary to.
    //     let links: Option<usize> = None;
    //     match check_mixing_strategy_compatibility(&mut atom_types, strategy, links) {
    //         Ok(_) => panic!("unexpected ok result"),
    //         Err(err) => match err {
    //             PotentialsInitializationError::InvalidMixingStrategy { strategy: _, msg: _ } => {},
    //             _ => panic!("unexpected error type")
    //         }
    //     }
    // }
}
