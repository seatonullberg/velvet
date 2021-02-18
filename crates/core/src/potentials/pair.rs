//! Potentials which operate on pairs of atoms.

use serde::{Deserialize, Serialize};

use crate::potentials::Potential;
use crate::system::elements::Element;
use crate::system::System;

/// Shared behavior for pair potentials.
#[typetag::serde(tag = "type")]
pub trait PairPotential: Potential {
    /// Returns the potential energy of an atom in a pair separated by a distance `r`.
    fn energy(&self, r: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r`.
    fn force(&self, r: f32) -> f32;
}

/// Metadata to define a unique pair type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PairMeta {
    /// Cutoff radius.
    pub cutoff: f32,
    /// Pair of elements.
    pub elements: (Element, Element),
    /// Indices of each target pair.
    pub indices: Vec<(usize, usize)>,
}

impl PairMeta {
    pub fn new(cutoff: f32, elements: (Element, Element), system: &System) -> PairMeta {
        let mut indices = Vec::with_capacity(system.size() * system.size());
        for i in 0..system.size() {
            for j in (i + 1)..system.size() {
                let elem_i = system.elements[i];
                let elem_j = system.elements[j];
                if (elem_i, elem_j) == elements {
                    indices.push((i, j));
                } else if (elem_j, elem_i) == elements {
                    indices.push((j, i))
                }
            }
        }
        indices.shrink_to_fit();

        PairMeta {
            cutoff,
            elements,
            indices,
        }
    }
}
/// Harmonic style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Harmonic {
    k: f32,
    x0: f32,
}

impl Harmonic {
    /// Returns a new Harmonic style pair potential.
    ///
    /// # Arguments
    ///
    /// * `k` - Spring constant
    /// * `x0` - Equilibrium displacement distance
    pub fn new(k: f32, x0: f32) -> Harmonic {
        Harmonic { k, x0 }
    }
}

#[typetag::serde]
impl Potential for Harmonic {}

#[typetag::serde]
impl PairPotential for Harmonic {
    fn energy(&self, r: f32) -> f32 {
        let dr = r - self.x0;
        self.k * dr * dr
    }

    fn force(&self, r: f32) -> f32 {
        2.0 * self.k * (r - self.x0)
    }
}

/// Lennard-Jones style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LennardJones {
    epsilon: f32,
    sigma: f32,
}

impl LennardJones {
    /// Returns a new Lennard-Jones style pair potential.
    ///
    /// # Arguments
    ///
    /// * `epsilon` - Depth of the potential well
    /// * `sigma` - Distance at which the pair potential energy is zero
    pub fn new(epsilon: f32, sigma: f32) -> LennardJones {
        LennardJones { epsilon, sigma }
    }
}

#[typetag::serde]
impl Potential for LennardJones {}

#[typetag::serde]
impl PairPotential for LennardJones {
    fn energy(&self, r: f32) -> f32 {
        let term = (self.sigma / r).powi(6);
        4.0 * self.epsilon * (term * term - term)
    }

    fn force(&self, r: f32) -> f32 {
        let term_a = (24.0 * self.sigma.powi(6)) / r.powi(7);
        let term_b = (48.0 * self.sigma.powi(12)) / r.powi(13);
        self.epsilon * (term_a - term_b)
    }
}

/// Mie style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Mie {
    epsilon: f32,
    sigma: f32,
    gamma_a: f32,
    gamma_r: f32,
}

impl Mie {
    /// Returns a new Mie style pair potential.
    ///
    /// # Arguments
    ///
    /// * `epsilon` - Depth of the potential well
    /// * `sigma` - Distance at which the pair potential energy is zero
    /// * `gamma_a` - Exponent on the attractive term
    /// * `gamma_r` - Exponent on the repulsive term
    pub fn new(epsilon: f32, sigma: f32, gamma_a: f32, gamma_r: f32) -> Mie {
        Mie {
            epsilon,
            sigma,
            gamma_a,
            gamma_r,
        }
    }
}

#[typetag::serde]
impl Potential for Mie {}

#[typetag::serde]
impl PairPotential for Mie {
    fn energy(&self, r: f32) -> f32 {
        let term_a = (self.sigma / r).powf(self.gamma_r);
        let term_b = (self.sigma / r).powf(self.gamma_a);
        let c = (self.gamma_r / (self.gamma_r - self.gamma_a))
            * (self.gamma_r / self.gamma_a).powf(self.gamma_a / (self.gamma_r - self.gamma_a));
        c * self.epsilon * (term_a - term_b)
    }

    fn force(&self, r: f32) -> f32 {
        let c = (self.gamma_r / (self.gamma_r - self.gamma_a))
            * (self.gamma_r / self.gamma_a).powf(self.gamma_a / (self.gamma_r - self.gamma_a));
        let term_a = (c * self.gamma_a * self.epsilon * (self.sigma / r).powf(self.gamma_a)) / r;
        let term_b = (c * self.gamma_r * self.epsilon * (self.sigma / r).powf(self.gamma_r)) / r;
        term_a - term_b
    }
}

/// Morse style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Morse {
    a: f32,
    d_e: f32,
    r_e: f32,
}

impl Morse {
    /// Returns a new Morse style pair potential.
    ///
    /// # Arguments
    ///
    /// * `a` - Width of the potential well
    /// * `d_e` - Depth of the potential well
    /// * `r_e` - Equilibrium bond distance
    pub fn new(a: f32, d_e: f32, r_e: f32) -> Morse {
        Morse { a, d_e, r_e }
    }
}

#[typetag::serde]
impl Potential for Morse {}

#[typetag::serde]
impl PairPotential for Morse {
    fn energy(&self, r: f32) -> f32 {
        let term_a = f32::exp(-2.0 * self.a * (r - self.r_e));
        let term_b = 2.0 * f32::exp(-self.a * (r - self.r_e));
        self.d_e * (term_a - term_b)
    }

    fn force(&self, r: f32) -> f32 {
        let term_a = f32::exp(-self.a * (r - self.r_e));
        let term_b = f32::exp(-2.0 * self.a * (r - self.r_e));
        2.0 * self.a * self.d_e * (term_a - term_b)
    }
}

#[cfg(test)]
mod tests {
    use super::{Harmonic, LennardJones, Mie, Morse, PairPotential};
    use approx::*;

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
        assert_relative_eq!(r0_energy, harmonic.energy(r0));
        assert_relative_eq!(r0_force, harmonic.force(r0));

        // test r1 energy and force
        let r1_energy = 0.0;
        let r1_force = 0.0;
        assert_relative_eq!(r1_energy, harmonic.energy(r1));
        assert_relative_eq!(r1_force, harmonic.force(r1));

        // test r1 energy and force
        let r2_energy = 12.5;
        let r2_force = 50.0;
        assert_relative_eq!(r2_energy, harmonic.energy(r2));
        assert_relative_eq!(r2_force, harmonic.force(r2));
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
        assert_relative_eq!(r0_energy, lj.energy(r0));
        assert_relative_eq!(r0_force, lj.force(r0));

        // test r1 energy and force
        let r1_energy = 0.0;
        let r1_force = -9.6;
        assert_relative_eq!(r1_energy, lj.energy(r1));
        assert_relative_eq!(r1_force, lj.force(r1));

        // test r2 energy and force
        let r2_energy = -0.89096529;
        let r2_force = 0.8846772;
        assert_relative_eq!(r2_energy, lj.energy(r2));
        assert_relative_eq!(r2_force, lj.force(r2));
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
        assert_relative_eq!(r0_energy, mie.energy(r0));
        assert_relative_eq!(r0_force, mie.force(r0));

        // test r1 energy and force
        let r1_energy = 0.0;
        let r1_force = -9.6;
        assert_relative_eq!(r1_energy, mie.energy(r1));
        assert_relative_eq!(r1_force, mie.force(r1));

        // test r2 energy and force
        let r2_energy = -0.89096529;
        let r2_force = 0.8846772;
        assert_relative_eq!(r2_energy, mie.energy(r2));
        assert_relative_eq!(r2_force, mie.force(r2));
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
        let r0_energy = 0.9907551;
        let r0_force = -28.376266;
        assert_relative_eq!(r0_energy, morse.energy(r0));
        assert_relative_eq!(r0_force, morse.force(r0));

        let r1_energy = -4.0;
        let r1_force = 0.0;
        assert_relative_eq!(r1_energy, morse.energy(r1));
        assert_relative_eq!(r1_force, morse.force(r1));

        let r2_energy = -2.8864117;
        let r2_force = 2.9908366;
        assert_relative_eq!(r2_energy, morse.energy(r2));
        assert_relative_eq!(r2_force, morse.force(r2));
    }
}
