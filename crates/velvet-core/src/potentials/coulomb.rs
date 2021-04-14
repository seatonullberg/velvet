//! Potentials which describe Coulombic electrostatic interactions.

#[cfg(feature = "f64")]
use libm::erfc;

#[cfg(not(feature = "f64"))]
use libm::erfcf as erfc;

use crate::internal::consts::PI;
use crate::internal::Float;
use crate::potentials::functions::Wolf;
use crate::potentials::Potential;

/// Shared behavior for coulomb potentials.
pub trait CoulombPotential: Potential {
    /// Returns the potential energy of an atom in a pair with charges `qi` and `qj` seperated by a distance `r`.
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float;
    /// Returns the potential energy of an atom with charge `qi` interacting with itself.
    fn energy_self(&self, qi: Float) -> Float;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r` with charges `qi` and `qj`.
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float;
}

impl CoulombPotential for Wolf {
    #[inline]
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        let term_a = erfc(self.alpha * r) / r;
        let term_b = erfc(self.alpha * self.cutoff) / self.cutoff;
        qi * qj * (term_a - term_b)
    }

    #[inline]
    fn energy_self(&self, qi: Float) -> Float {
        let term_a = 0.5 * erfc(self.alpha * self.cutoff) / self.cutoff;
        let term_b = self.alpha / Float::sqrt(PI);
        -qi.powi(2) * (term_a + term_b)
    }

    #[inline]
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        let r2 = r * r;
        let term_a = -erfc(self.alpha * r) / r2;
        let term_b =
            2.0 * self.alpha * Float::exp(-(self.alpha * self.alpha * r2)) / (Float::sqrt(PI) * r);
        qi * qj * (term_a - term_b)
    }
}

#[cfg(test)]
mod tests {
    use super::{CoulombPotential, Wolf};
    use approx::*;

    #[test]
    fn wolf() {
        let wolf = Wolf::new(0.25, 8.0);

        let energy_wolf = wolf.energy(1.0, -1.0, 1.5);
        let energy_target = -0.396671;
        assert_relative_eq!(energy_wolf, energy_target, epsilon = 1e-5);

        let force_wolf = wolf.force(1.0, -1.0, 1.5);
        let force_target = 0.428229;
        assert_relative_eq!(force_wolf, force_target, epsilon = 1e-5);

        let energy_self_wolf = wolf.energy_self(1.0);
        let energy_self_target = -0.141340;
        assert_relative_eq!(energy_self_wolf, energy_self_target, epsilon = 1e-5);
    }
}
