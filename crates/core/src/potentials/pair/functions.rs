use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::potentials::pair::PairPotential;
use crate::potentials::Potential;

/// Harmonic style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Harmonic {
    k: Float,
    x0: Float,
}

impl Harmonic {
    /// Returns a new Harmonic style pair potential.
    ///
    /// # Arguments
    ///
    /// * `k` - Spring constant
    /// * `x0` - Equilibrium displacement distance
    pub fn new(k: Float, x0: Float) -> Harmonic {
        Harmonic { k, x0 }
    }
}

#[typetag::serde]
impl Potential for Harmonic {}

#[typetag::serde]
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

/// Lennard-Jones style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LennardJones {
    epsilon: Float,
    sigma: Float,
}

impl LennardJones {
    /// Returns a new Lennard-Jones style pair potential.
    ///
    /// # Arguments
    ///
    /// * `epsilon` - Depth of the potential well
    /// * `sigma` - Distance at which the pair potential energy is zero
    pub fn new(epsilon: Float, sigma: Float) -> LennardJones {
        LennardJones { epsilon, sigma }
    }
}

#[typetag::serde]
impl Potential for LennardJones {}

#[typetag::serde]
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

/// Mie style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Mie {
    epsilon: Float,
    sigma: Float,
    gamma_a: Float,
    gamma_r: Float,
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
    pub fn new(epsilon: Float, sigma: Float, gamma_a: Float, gamma_r: Float) -> Mie {
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

/// Morse style pair potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Morse {
    a: Float,
    d_e: Float,
    r_e: Float,
}

impl Morse {
    /// Returns a new Morse style pair potential.
    ///
    /// # Arguments
    ///
    /// * `a` - Width of the potential well
    /// * `d_e` - Depth of the potential well
    /// * `r_e` - Equilibrium bond distance
    pub fn new(a: Float, d_e: Float, r_e: Float) -> Morse {
        Morse { a, d_e, r_e }
    }
}

#[typetag::serde]
impl Potential for Morse {}

#[typetag::serde]
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