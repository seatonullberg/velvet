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
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PairMeta {
    /// Cutoff radius.
    pub cutoff: f32,
    /// Pair of elements.
    pub elements: (Element, Element),
}

impl PairMeta {
    /// Returns a new `PairMeta`.
    pub fn new(cutoff: f32, elements: (Element, Element)) -> PairMeta {
        PairMeta { cutoff, elements }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PairDescriptor {
    pub potential: Box<dyn PairPotential>,
    pub meta: PairMeta,
    pub indices: Vec<(usize, usize)>,
}

impl PairDescriptor {
    pub fn new(
        potential: Box<dyn PairPotential>,
        meta: PairMeta,
        system: &System,
    ) -> PairDescriptor {
        let mut indices = Vec::with_capacity(2 * system.size());
        for i in 0..system.size() {
            for j in (i + 1)..system.size() {
                let elem_i = system.iter_elements().nth(i).unwrap();
                let elem_j = system.iter_elements().nth(j).unwrap();
                if (*elem_i, *elem_j) == meta.elements {
                    indices.push((i, j));
                }
            }
        }
        indices.shrink_to_fit();

        PairDescriptor {
            potential,
            meta,
            indices,
        }
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
        let term_a = (48.0 * self.sigma.powi(12) * self.epsilon) / r.powi(13);
        let term_b = (24.0 * self.sigma.powi(6) * self.epsilon) / r.powi(7);
        term_a - term_b
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
        0.5 * self.k * dr * dr
    }

    fn force(&self, r: f32) -> f32 {
        self.k * (self.x0 - r)
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
        let term_a = (c * self.gamma_r * self.epsilon * (self.sigma / r).powf(self.gamma_r)) / r;
        let term_b = (c * self.gamma_a * self.epsilon * (self.sigma / r).powf(self.gamma_a)) / r;
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
        let term_a = f32::exp(-2.0 * self.a * (r - self.r_e));
        let term_b = f32::exp(-self.a * (r - self.r_e));
        2.0 * self.a * self.d_e * (term_a - term_b)
    }
}

#[cfg(test)]
mod tests {
    use super::{Harmonic, LennardJones, Mie, Morse, PairPotential};
    use approx::*;

    #[test]
    fn lennard_jones() {
        let lj = LennardJones::new(0.8, 2.0);
        assert_relative_eq!(lj.energy(2.0), 0.0);
        assert_relative_eq!(lj.energy(2.5), -0.6189586);
        assert_relative_eq!(
            lj.force(f32::powf(2.0, 1.0 / 6.0) * 2.0).abs(),
            0.0,
            epsilon = 1e-6
        );
        assert_relative_eq!(lj.force(2.5), -0.9577348);
    }

    #[test]
    fn harmonic() {
        let harm = Harmonic::new(50.0, 2.0);
        assert_relative_eq!(harm.energy(2.0), 0.0);
        assert_relative_eq!(harm.energy(2.5), 6.25);
        assert_relative_eq!(harm.force(2.0), 0.0);
        assert_relative_eq!(harm.force(2.5), -25.0);
    }

    #[test]
    fn mie() {
        let mie = Mie::new(0.8, 2.0, 6.0, 12.0);
        assert_relative_eq!(mie.energy(2.0), 0.0);
        assert_relative_eq!(mie.energy(2.5), -0.61895853);
        assert_relative_eq!(
            mie.force(f32::powf(2.0, 1.0 / 6.0) * 2.0).abs(),
            0.0,
            epsilon = 1e-5
        );
        assert_relative_eq!(mie.force(2.5), -0.9577347);
    }

    #[test]
    fn morse() {
        let morse = Morse::new(1.3, 4.0, 2.0);
        assert_relative_eq!(morse.energy(2.0), -4.0);
        assert_relative_eq!(morse.force(2.0), 0.0);
    }
}
