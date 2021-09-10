//! Implementations of potentials which describe electrostatic interactions.

#[cfg(feature = "f64")]
use libm::erfc as erfc;

#[cfg(not(feature = "f64"))]
use libm::erfcf as erfc;

use crate::internal::consts::COULOMB;
use crate::internal::Float;
use crate::internal::consts::FRAC_2_SQRT_PI;
use crate::potentials::{DampedShiftedForce, StandardCoulombic};
use crate::potentials::Potential;
use crate::selection::{setup_pairs_with_charge, update_pairs_by_cutoff_radius, Selection};
use crate::system::System;

/// Shared behavior for Coulombic potentials.
pub trait CoulombPotential: Potential {
    /// Returns the potential energy of an atom in a pair with charges `qi` and `qj` seperated by a distance `r`.
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r` with charges `qi` and `qj`.
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float;
}

impl CoulombPotential for DampedShiftedForce {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        let factor = FRAC_2_SQRT_PI * self.alpha;
        let alpha2 = self.alpha.powi(2);
        let cutoff2 = self.cutoff.powi(2);

        let term_a = erfc(self.alpha * r) / r;
        let term_b = erfc(self.alpha * self.cutoff) / self.cutoff;
        let term_c = erfc(self.alpha * self.cutoff) / cutoff2;
        let term_d = factor * (Float::exp(-alpha2 * cutoff2) / self.cutoff);
        let term_e = r - self.cutoff;

        qi * qj * (term_a - term_b + (term_c + term_d) * term_e)
    }

    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        let factor = FRAC_2_SQRT_PI * self.alpha;
        let r2 = r.powi(2);
        let alpha2 = self.alpha.powi(2);
        let cutoff2 = self.cutoff.powi(2);

        let term_a = erfc(self.alpha * r) / r2;
        let term_b = factor * Float::exp(-alpha2 * r2) / r;
        let term_c = erfc(self.alpha * self.cutoff) / cutoff2;
        let term_d = factor * Float::exp(-alpha2 * cutoff2) / self.cutoff;

        qi * qj * ((term_a + term_b) - (term_c + term_d))
    }
}

impl CoulombPotential for StandardCoulombic {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        (COULOMB * qi * qj) / (self.dielectric * r)
    }

    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        -(COULOMB * qi * qj) / (self.dielectric * r.powi(2))
    }
}

type CoulombSetupFn = fn(&System, ()) -> Vec<[usize; 2]>;

type CoulombUpdateFn = fn(&System, &[[usize; 2]], Float) -> Vec<[usize; 2]>;

type CoulombSelection = Selection<CoulombSetupFn, (), CoulombUpdateFn, Float, 2>;

pub(crate) struct CoulombPotentialMeta {
    pub potential: Box<dyn CoulombPotential>,
    pub cutoff: Float,
    pub thickness: Float,
    pub selection: CoulombSelection,
}

impl CoulombPotentialMeta {
    pub fn new<T>(potential: T, cutoff: Float, thickness: Float) -> CoulombPotentialMeta
    where
        T: CoulombPotential + 'static,
    {
        let selection = Selection::new(
            setup_pairs_with_charge as CoulombSetupFn,
            update_pairs_by_cutoff_radius as CoulombUpdateFn,
        );
        CoulombPotentialMeta {
            potential: Box::new(potential),
            selection,
            cutoff,
            thickness,
        }
    }

    pub fn setup(&mut self, system: &System) {
        self.selection.setup(system, ())
    }

    pub fn update(&mut self, system: &System) {
        self.selection.update(system, self.cutoff + self.thickness)
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
