//! Lennard-Jones 12-6 potential.

use crate::errors::PotentialsInitializationError;
use crate::potentials::internal::check_parameter_names;
use crate::potentials::pair::PairPotential;
use crate::potentials::Potential;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default)]
pub struct LennardJones {
    epsilon: f64,
    sigma: f64,
}

impl LennardJones {
    pub fn new(epsilon: f64, sigma: f64) -> Self {
        LennardJones { epsilon, sigma }
    }
}

impl TryFrom<&HashMap<&str, f64>> for LennardJones {
    type Error = PotentialsInitializationError;

    fn try_from(value: &HashMap<&str, f64>) -> Result<Self, Self::Error> {
        let given_names: Vec<String> = value.keys().map(|&s| s.to_string()).collect();
        let required_names: Vec<String> = Self::parameter_names();
        // Check that given parameter names match required parameter names.
        // Return an error if they do not.
        check_parameter_names(&given_names, &required_names)?;
        // The names have already been checked so the unwrap is safe here.
        let &epsilon = value.get("epsilon").unwrap();
        let &sigma = value.get("sigma").unwrap();
        Ok(LennardJones { epsilon, sigma })
    }
}

impl<'a> Potential<'a> for LennardJones {
    fn parameter_names() -> Vec<String> {
        vec!["epsilon".to_string(), "sigma".to_string()]
    }

    fn parameters(&self) -> HashMap<String, f64> {
        HashMap::from([
            ("epsilon".to_string(), self.epsilon),
            ("sigma".to_string(), self.sigma),
        ])
    }
}

impl<'a> PairPotential<'a> for LennardJones {
    fn energy(&self, r: f64) -> f64 {
        let term = (self.sigma / r).powi(6);
        4.0 * self.epsilon * (term * term - term)
    }

    fn force(&self, r: f64) -> f64 {
        let term_a = (24.0 * self.sigma.powi(6)) / r.powi(7);
        let term_b = (48.0 * self.sigma.powi(12)) / r.powi(13);
        self.epsilon * (term_a - term_b)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::potentials::pair::PairPotential;

    use super::LennardJones;

    use approx::assert_relative_eq;

    struct Target {
        distance: f64,
        energy: f64,
        force: f64,
    }

    impl Target {
        fn new(distance: f64, energy: f64, force: f64) -> Target {
            Target {
                distance,
                energy,
                force,
            }
        }
    }

    // Check that the LennardJones potential produces expected energies and forces.
    #[test]
    fn lennard_jones() {
        // Initialize the potential.
        let parameters: HashMap<&str, f64> = HashMap::from([("epsilon", 1.0), ("sigma", 2.5)]);
        let lj = LennardJones::try_from(&parameters).unwrap();

        // Define some targets to test.
        let targets = vec![
            Target::new(2.0, 42.95, -303.47),
            Target::new(2.5, 0.0, -9.6),
            Target::new(3.0, -0.89, 0.88),
        ];

        // Test each target.
        for target in targets {
            let r = target.distance;
            let energy = lj.energy(r);
            let force = lj.force(r);
            assert_relative_eq!(energy, target.energy, epsilon = 0.01);
            assert_relative_eq!(force, target.force, epsilon = 0.01);
        }
    }
}
