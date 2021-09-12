//! Implementations of potentials which describe electrostatic interactions.

#[cfg(feature = "f64")]
use libm::erfc;

#[cfg(not(feature = "f64"))]
use libm::erfcf as erfc;

use crate::internal::consts::SQRT_PI;
use crate::internal::Float;
use crate::potentials::Potential;
use crate::potentials::Wolf;
use crate::selection::{setup_pairs_with_charge, update_pairs_by_cutoff_radius, Selection};
use crate::system::System;

/// Shared behavior for Coulombic potentials.
pub trait CoulombPotential: Potential {
    /// Returns the potential energy of an atom in a pair with charges `qi` and `qj` seperated by a distance `r`.
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float;
    /// Returns the potential energy of an atom due to its own charge.
    fn self_energy(&self, qi: Float) -> Float;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r` with charges `qi` and `qj`.
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float;
}

impl CoulombPotential for Wolf {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        let term_a = erfc(self.alpha * r) / r;
        let term_b = erfc(self.alpha * self.cutoff) / self.cutoff;
        qi * qj * (term_a - term_b)
    }

    fn self_energy(&self, qi: Float) -> Float {
        let term_a = 0.5 * (erfc(self.alpha * self.cutoff) / self.cutoff);
        let term_b = self.alpha / SQRT_PI;
        -(qi.powi(2)) * (term_a + term_b)
    }

    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        let term_a = -erfc(self.alpha * r) / r.powi(2);
        let term_b = 2.0 * self.alpha * Float::exp(self.alpha.powi(2) * -r.powi(2));
        let term_c = SQRT_PI * r;
        qi * qj * (term_a - (term_b / term_c))
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
    use super::{CoulombPotential, Wolf};
    use approx::*;

    #[test]
    fn wolf() {
        let wolf = Wolf::new(0.1, 10.0);
        let qi = 2.0;
        let qj = 3.0;
        let r0 = 2.5;
        let r1 = 5.0;
        let r2 = 10.0;

        // test self energy
        let qi_self_energy = -0.2571357;
        let qj_self_energy = -0.5785553;
        assert_relative_eq!(qi_self_energy, wolf.self_energy(qi), epsilon = 1e-3);
        assert_relative_eq!(qj_self_energy, wolf.self_energy(qj), epsilon = 1e-3);

        // test r0 energy and force
        let r0_energy = 1.642437;
        let r0_force = -0.94913006;
        assert_relative_eq!(r0_energy, wolf.energy(qi, qj, r0));
        assert_relative_eq!(r0_force, wolf.force(qi, qj, r0));

        // test r0 energy and force
        let r1_energy = 0.48102063;
        let r1_force = -0.22053395;
        assert_relative_eq!(r1_energy, wolf.energy(qi, qj, r1));
        assert_relative_eq!(r1_force, wolf.force(qi, qj, r1));

        // test r0 energy and force
        let r2_energy = 0.0;
        let r2_force = -0.034344397;
        assert_relative_eq!(r2_energy, wolf.energy(qi, qj, r2));
        assert_relative_eq!(r2_force, wolf.force(qi, qj, r2));
    }
}
