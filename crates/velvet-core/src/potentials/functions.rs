//! Interatomic potential functions.

use crate::internal::Float;
use crate::potentials::Potential;

#[derive(Clone, Copy, Debug)]
pub struct Buckingham {
    pub a: Float,
    pub rho: Float,
    pub c: Float,
}

impl Buckingham {
    /// Returns a new Buckingham style potential
    pub fn new(a: Float, rho: Float, c: Float) -> Buckingham {
        Buckingham {a, rho, c}
    }
}

impl Potential for Buckingham {}

/// Harmonic style pair potential.
#[derive(Clone, Copy, Debug)]
pub struct Harmonic {
    /// Spring constant.
    pub k: Float,
    /// Equilibrium displacement distance.
    pub x0: Float,
}

impl Harmonic {
    /// Returns a new Harmonic style pair potential.
    pub fn new(k: Float, x0: Float) -> Harmonic {
        Harmonic { k, x0 }
    }
}

impl Potential for Harmonic {}

/// Lennard-Jones style pair potential.
#[derive(Clone, Copy, Debug)]
pub struct LennardJones {
    /// Depth of the potential well.
    pub epsilon: Float,
    /// Distance at which the pair potential energy is zero.
    pub sigma: Float,
}

impl LennardJones {
    /// Returns a new Lennard-Jones style pair potential.
    pub fn new(epsilon: Float, sigma: Float) -> LennardJones {
        LennardJones { epsilon, sigma }
    }
}

impl Potential for LennardJones {}

/// Mie style pair potential.
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    /// Depth of the potential well.
    pub epsilon: Float,
    /// Distance at which the pair potential energy is zero.
    pub sigma: Float,
    /// Exponent on the attractive term.
    pub gamma_a: Float,
    /// Exponent on the repulsize term.
    pub gamma_r: Float,
}

impl Mie {
    /// Returns a new Mie style pair potential.
    pub fn new(epsilon: Float, sigma: Float, gamma_a: Float, gamma_r: Float) -> Mie {
        Mie {
            epsilon,
            sigma,
            gamma_a,
            gamma_r,
        }
    }
}

impl Potential for Mie {}

/// Morse style pair potential.
#[derive(Clone, Copy, Debug)]
pub struct Morse {
    /// Width of the potential well.
    pub a: Float,
    /// Depth of the potential well.
    pub d_e: Float,
    /// Equilibrium bond distance.
    pub r_e: Float,
}

impl Morse {
    /// Returns a new Morse style pair potential.
    pub fn new(a: Float, d_e: Float, r_e: Float) -> Morse {
        Morse { a, d_e, r_e }
    }
}

impl Potential for Morse {}

/// Wolf style coulomb potential.
#[derive(Clone, Copy, Debug)]
pub struct Wolf {
    /// Damping parameter.
    pub alpha: Float,
    /// Cutoff radius.
    pub cutoff: Float,
}

impl Wolf {
    /// Returns a new Wolf style coulomb potential.
    pub fn new(alpha: Float, cutoff: Float) -> Wolf {
        Wolf { alpha, cutoff }
    }
}

impl Potential for Wolf {}
