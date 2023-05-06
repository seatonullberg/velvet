use crate::errors::PotentialsInitializationError;

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
}
