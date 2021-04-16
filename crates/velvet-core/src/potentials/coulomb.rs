//! Potentials which describe Coulombic electrostatic interactions.

use crate::internal::consts::COULOMB;
use crate::internal::Float;
use crate::potentials::functions::StandardCoulombic;
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

pub(crate) struct CoulombPotentials {
    pub potentials: Vec<Box<dyn CoulombPotential>>,
    pub selections: Vec<CoulombSelection>,
    pub cutoffs: Vec<Float>,
    pub thicknesses: Vec<Float>,
    pub update_frequency: usize,
}

impl CoulombPotentials {
    pub fn setup(&mut self, system: &System) {
        self.selections
            .iter_mut()
            .for_each(|selection| selection.setup(system, ()));
    }

    pub fn update(&mut self, system: &System) {
        self.selections
            .iter_mut()
            .zip(self.cutoffs.iter())
            .zip(self.thicknesses.iter())
            .for_each(|((selection, cutoff), thickness)| {
                selection.update(system, cutoff + thickness)
            })
    }
}

pub(crate) struct CoulombPotentialsBuilder {
    potentials: Vec<Box<dyn CoulombPotential>>,
    selections: Vec<CoulombSelection>,
    cutoffs: Vec<Float>,
    thicknesses: Vec<Float>,
    update_frequency: usize,
}

impl CoulombPotentialsBuilder {
    pub fn new() -> CoulombPotentialsBuilder {
        CoulombPotentialsBuilder {
            potentials: Vec::new(),
            selections: Vec::new(),
            cutoffs: Vec::new(),
            thicknesses: Vec::new(),
            update_frequency: 1,
        }
    }

    pub fn update_frequency(mut self, freq: usize) -> CoulombPotentialsBuilder {
        self.update_frequency = freq;
        self
    }

    pub fn coulomb<P: CoulombPotential + 'static>(
        mut self,
        potential: P,
        cutoff: Float,
        thickness: Float,
    ) -> CoulombPotentialsBuilder {
        let potential = Box::new(potential);
        self.potentials.push(potential);
        let selection = Selection::new(
            setup_pairs_with_charge as CoulombSetupFn,
            update_pairs_by_cutoff_radius as CoulombUpdateFn,
        );
        self.selections.push(selection);
        self.cutoffs.push(cutoff);
        self.thicknesses.push(thickness);
        self
    }

    pub fn build(self) -> CoulombPotentials {
        CoulombPotentials {
            potentials: self.potentials,
            selections: self.selections,
            cutoffs: self.cutoffs,
            thicknesses: self.thicknesses,
            update_frequency: self.update_frequency,
        }
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
