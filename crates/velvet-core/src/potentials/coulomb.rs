//! Potentials which describe Coulombic electrostatic interactions.

use crate::internal::consts::COULOMB;
use crate::internal::Float;
use crate::potentials::functions::StandardCoulombic;
use crate::potentials::Potential;

/// Shared behavior for Coulombic potentials.
pub trait CoulombPotential: Potential {
    /// Returns the potential energy of an atom in a pair with charges `qi` and `qj` seperated by a distance `r`.
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r` with charges `qi` and `qj`.
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float;
}

impl CoulombPotential for StandardCoulombic {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        (COULOMB * qi * qj) / (self.dielectric * r)
    }

    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        -(COULOMB * qi * qj) / (self.dielectric * r.powi(2))
    }
}

#[cfg(test)]
mod tests {
    use super::{CoulombPotential, StandardCoulombic};
    use approx::*;

    #[test]
    fn standard_coulombic() {
        // initialize the potential
        let dielectric = 1.0;
        let coulombic = StandardCoulombic::new(dielectric);
        let qi = 2.0;
        let qj = 3.0;
        let r0 = 1.0;
        let r1 = 2.5;
        let r2 = 5.0;

        // test r0 energy and force
        let r0_energy = 1992.3816;
        let r0_force = -1992.3816;
        assert_relative_eq!(r0_energy, coulombic.energy(qi, qj, r0), epsilon = 1e-3);
        assert_relative_eq!(r0_force, coulombic.force(qi, qj, r0), epsilon = 1e-3);

        // test r1 energy and force
        let r1_energy = 796.95264;
        let r1_force = -318.781056;
        assert_relative_eq!(r1_energy, coulombic.energy(qi, qj, r1), epsilon = 1e-3);
        assert_relative_eq!(r1_force, coulombic.force(qi, qj, r1), epsilon = 1e-3);

        // test r2 energy and force
        let r2_energy = 398.47632;
        let r2_force = -79.695264;
        assert_relative_eq!(r2_energy, coulombic.energy(qi, qj, r2), epsilon = 1e-3);
        assert_relative_eq!(r2_force, coulombic.force(qi, qj, r2), epsilon = 1e-3);
    }
}
