//! Potentials which describe pairwise nonbonded interactions..

use crate::internal::Float;
use crate::neighbors::NeighborList;
use crate::potentials::functions::{Buckingham, Harmonic, LennardJones, Mie, Morse};
use crate::potentials::Potential;
use crate::system::particle::ParticleType;
use crate::system::System;

/// Shared behavior for pair potentials.
pub trait PairPotential: Potential {
    /// Returns the potential energy of an atom in a pair separated by a distance `r`.
    fn energy(&self, r: Float) -> Float;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r`.
    fn force(&self, r: Float) -> Float;
}

impl PairPotential for Buckingham {
    #[inline]
    fn energy(&self, r: Float) -> Float {
        self.a * Float::exp(-r / self.rho) - (self.c / r.powi(6))
    }

    #[inline]
    fn force(&self, r: Float) -> Float {
        let term_a = (6.0 * self.c) / r.powi(7);
        let term_b = (self.a * Float::exp(-r / self.rho)) / self.rho;
        term_a - term_b
    }
}

impl PairPotential for Harmonic {
    #[inline]
    fn energy(&self, r: Float) -> Float {
        let dr = r - self.x0;
        self.k * dr * dr
    }

    #[inline]
    fn force(&self, r: Float) -> Float {
        2.0 * self.k * (r - self.x0)
    }
}

impl PairPotential for LennardJones {
    #[inline]
    fn energy(&self, r: Float) -> Float {
        let term = (self.sigma / r).powi(6);
        4.0 * self.epsilon * (term * term - term)
    }

    #[inline]
    fn force(&self, r: Float) -> Float {
        let term_a = (24.0 * self.sigma.powi(6)) / r.powi(7);
        let term_b = (48.0 * self.sigma.powi(12)) / r.powi(13);
        self.epsilon * (term_a - term_b)
    }
}

impl PairPotential for Mie {
    #[inline]
    fn energy(&self, r: Float) -> Float {
        let term_a = (self.sigma / r).powf(self.gamma_r);
        let term_b = (self.sigma / r).powf(self.gamma_a);
        let c = (self.gamma_r / (self.gamma_r - self.gamma_a))
            * (self.gamma_r / self.gamma_a).powf(self.gamma_a / (self.gamma_r - self.gamma_a));
        c * self.epsilon * (term_a - term_b)
    }

    #[inline]
    fn force(&self, r: Float) -> Float {
        let c = (self.gamma_r / (self.gamma_r - self.gamma_a))
            * (self.gamma_r / self.gamma_a).powf(self.gamma_a / (self.gamma_r - self.gamma_a));
        let term_a = (c * self.gamma_a * self.epsilon * (self.sigma / r).powf(self.gamma_a)) / r;
        let term_b = (c * self.gamma_r * self.epsilon * (self.sigma / r).powf(self.gamma_r)) / r;
        term_a - term_b
    }
}

impl PairPotential for Morse {
    #[inline]
    fn energy(&self, r: Float) -> Float {
        let term_a = Float::exp(-2.0 * self.a * (r - self.r_e));
        let term_b = 2.0 * Float::exp(-self.a * (r - self.r_e));
        self.d_e * (term_a - term_b)
    }

    #[inline]
    fn force(&self, r: Float) -> Float {
        let term_a = Float::exp(-self.a * (r - self.r_e));
        let term_b = Float::exp(-2.0 * self.a * (r - self.r_e));
        2.0 * self.a * self.d_e * (term_a - term_b)
    }
}

pub(crate) struct PairPotentials {
    pub potentials: Vec<Box<dyn PairPotential>>,
    pub neighbor_lists: Vec<NeighborList>,
    pub cutoffs: Vec<Float>,
    pub update_frequency: usize,
}

impl PairPotentials {
    pub fn setup(&mut self, system: &System) {
        self.neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.setup(system));
    }

    pub fn update(&mut self, system: &System) {
        // update neighbor lists
        self.neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.update(system));
    }
}

/// Convenient constructor for [`PairPotentials`].
pub(crate) struct PairPotentialsBuilder {
    potentials: Vec<Box<dyn PairPotential>>,
    neighbor_lists: Vec<NeighborList>,
    cutoffs: Vec<Float>,
    update_frequency: usize,
}

impl PairPotentialsBuilder {
    /// Returns a new `PairPotentialsBuilder`.
    pub fn new() -> PairPotentialsBuilder {
        PairPotentialsBuilder {
            potentials: Vec::new(),
            neighbor_lists: Vec::new(),
            cutoffs: Vec::new(),
            update_frequency: 1,
        }
    }

    /// Adds a new potential to the collection.
    pub fn pair<P>(
        mut self,
        potential: P,
        particle_types: (ParticleType, ParticleType),
        cutoff: Float,
        thickness: Float,
    ) -> PairPotentialsBuilder
    where
        P: PairPotential + 'static,
    {
        let potential = Box::new(potential);
        self.potentials.push(potential);
        let neighbor_list = NeighborList::new(cutoff + thickness, Some(particle_types));
        self.neighbor_lists.push(neighbor_list);
        self.cutoffs.push(cutoff);
        self
    }

    /// Sets the number of iterations between each call to `update`.
    pub fn update_frequency(mut self, freq: usize) -> PairPotentialsBuilder {
        self.update_frequency = freq;
        self
    }

    /// Consumes the builder and returns a new [`PairPotentials`] object.
    pub fn build(self) -> PairPotentials {
        PairPotentials {
            potentials: self.potentials,
            neighbor_lists: self.neighbor_lists,
            cutoffs: self.cutoffs,
            update_frequency: self.update_frequency,
        }
    }
}

impl Default for PairPotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Buckingham, Harmonic, LennardJones, Mie, Morse, PairPotential};
    use approx::*;

    #[test]
    fn buckingham() {
        // initialize the potential
        let a = 10_000.0;
        let rho = 2.0;
        let c = 100.0;
        let buckingham = Buckingham::new(a, rho, c);
        let r0 = 1.5;
        let r1 = 2.0;
        let r2 = 2.5;

        // test r0 energy and force
        let r0_energy = 4714.886378;
        let r0_force = -2326.716166;
        assert_relative_eq!(r0_energy, buckingham.energy(r0), epsilon = 1e-5);
        assert_relative_eq!(r0_force, buckingham.force(r0), epsilon = 1e-5);

        // test r1 energy and force
        let r1_energy = 3677.231912;
        let r1_force = -1834.709706;
        assert_relative_eq!(r1_energy, buckingham.energy(r1), epsilon = 1e-5);
        assert_relative_eq!(r1_force, buckingham.force(r1), epsilon = 1e-5);

        // test r2 energy and force
        let r2_energy = 2864.638369;
        let r2_force = -1431.540944;
        assert_relative_eq!(r2_energy, buckingham.energy(r2), epsilon = 1e-5);
        assert_relative_eq!(r2_force, buckingham.force(r2), epsilon = 1e-5);
    }

    #[test]
    fn harmonic() {
        // initialize the potantial
        let k = 50.0;
        let x0 = 2.0;
        let harmonic = Harmonic::new(k, x0);
        let r0 = 1.5;
        let r1 = 2.0;
        let r2 = 2.5;

        // test r0 energy and force
        let r0_energy = 12.5;
        let r0_force = -50.0;
        assert_relative_eq!(r0_energy, harmonic.energy(r0), epsilon = 1e-5);
        assert_relative_eq!(r0_force, harmonic.force(r0), epsilon = 1e-5);

        // test r1 energy and force
        let r1_energy = 0.0;
        let r1_force = 0.0;
        assert_relative_eq!(r1_energy, harmonic.energy(r1), epsilon = 1e-5);
        assert_relative_eq!(r1_force, harmonic.force(r1), epsilon = 1e-5);

        // test r1 energy and force
        let r2_energy = 12.5;
        let r2_force = 50.0;
        assert_relative_eq!(r2_energy, harmonic.energy(r2), epsilon = 1e-5);
        assert_relative_eq!(r2_force, harmonic.force(r2), epsilon = 1e-5);
    }

    #[test]
    fn lennard_jones() {
        // initialize the potential
        let epsilon = 1.0;
        let sigma = 2.5;
        let lj = LennardJones::new(epsilon, sigma);
        let r0 = 2.0;
        let r1 = 2.5;
        let r2 = 3.0;

        // test r0 energy and force
        let r0_energy = 42.948872;
        let r0_force = -303.469598;
        assert_relative_eq!(r0_energy, lj.energy(r0), epsilon = 1e-5);
        assert_relative_eq!(r0_force, lj.force(r0), epsilon = 1e-5);

        // test r1 energy and force
        let r1_energy = 0.0;
        let r1_force = -9.6;
        assert_relative_eq!(r1_energy, lj.energy(r1), epsilon = 1e-5);
        assert_relative_eq!(r1_force, lj.force(r1), epsilon = 1e-5);

        // test r2 energy and force
        let r2_energy = -0.89096529;
        let r2_force = 0.8846772;
        assert_relative_eq!(r2_energy, lj.energy(r2), epsilon = 1e-5);
        assert_relative_eq!(r2_force, lj.force(r2), epsilon = 1e-5);
    }

    #[test]
    fn mie() {
        let epsilon = 1.0;
        let sigma = 2.5;
        let gamma_a = 6.0;
        let gamma_r = 12.0;
        let mie = Mie::new(epsilon, sigma, gamma_a, gamma_r);

        let r0 = 2.0;
        let r1 = 2.5;
        let r2 = 3.0;

        // test r0 energy and force
        let r0_energy = 42.948872;
        let r0_force = -303.469598;
        assert_relative_eq!(r0_energy, mie.energy(r0), epsilon = 1e-5);
        assert_relative_eq!(r0_force, mie.force(r0), epsilon = 1e-5);

        // test r1 energy and force
        let r1_energy = 0.0;
        let r1_force = -9.6;
        assert_relative_eq!(r1_energy, mie.energy(r1), epsilon = 1e-5);
        assert_relative_eq!(r1_force, mie.force(r1), epsilon = 1e-5);

        // test r2 energy and force
        let r2_energy = -0.89096529;
        let r2_force = 0.8846772;
        assert_relative_eq!(r2_energy, mie.energy(r2), epsilon = 1e-5);
        assert_relative_eq!(r2_force, mie.force(r2), epsilon = 1e-5);
    }

    #[test]
    fn morse() {
        let a = 1.5;
        let d_e = 4.0;
        let r_e = 2.0;
        let morse = Morse::new(a, d_e, r_e);

        let r0 = 1.5;
        let r1 = 2.0;
        let r2 = 2.5;

        // test r0 energy and force
        let r0_energy = 0.9907561;
        let r0_force = -28.376268;
        assert_relative_eq!(r0_energy, morse.energy(r0), epsilon = 1e-5);
        assert_relative_eq!(r0_force, morse.force(r0), epsilon = 1e-5);

        let r1_energy = -4.0;
        let r1_force = 0.0;
        assert_relative_eq!(r1_energy, morse.energy(r1), epsilon = 1e-5);
        assert_relative_eq!(r1_force, morse.force(r1), epsilon = 1e-5);

        let r2_energy = -2.8864117;
        let r2_force = 2.9908366;
        assert_relative_eq!(r2_energy, morse.energy(r2), epsilon = 1e-5);
        assert_relative_eq!(r2_force, morse.force(r2), epsilon = 1e-5);
    }
}
