use crate::energy::EnergyEvaluator;
use crate::force::ForceEvaluator;
use crate::system::System;

/// Lennard-Jones potential
///
/// $$
/// V(r) = 4\epsilon \Bigg[\Big(\frac \sigma r\Big)^{12} - \Big(\frac \sigma r\Big)^6\Bigg]
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct LennardJones {
    /// Depth of the potential well.
    epsilon: f32,
    /// Finite distance at which the potential evaluates to zero.
    sigma: f32,
}

/// Mie potential
///
/// $$
/// V(r) = C\epsilon\Big[\Big(\frac \sigma r\Big)^{\gamma_r} - \Big(\frac \sigma r\Big)^{\gamma_a}\Big]
/// $$
///
/// Where C is a function:
///
/// $$
/// C = \Bigg(\frac {\gamma_r} {\gamma_r - \gamma_a}\Bigg)\Bigg(\frac {\gamma_r} {\gamma_a}\Bigg)^{\big(\frac {\gamma_a} {\gamma_r - \gamma_a}\big)}
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Mie {
    epsilon: f32,
    sigma: f32,
    gamma_a: f32,
    gamma_r: f32,
}

/// Morse potential
///
/// $$
/// V(r) = D_e\Big(e^{-2a(r - r_e)} - 2e^{-a(r - r_e)}\Big)
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Morse {
    /// Potential well width.
    a: f32,
    /// Potential well depth.
    d_e: f32,
    /// Equilibrium bond distance.
    r_e: f32,
}

/// Potentials which take a pairwise distance as their only argument.
#[derive(Clone, Copy, Debug)]
pub enum PairPotential {
    LennardJones(LennardJones),
    Mie(Mie),
    Morse(Morse),
}

impl PairPotential {
    /// Returns the potential energy of a pair interaction.
    pub fn energy(&self, r: f32) -> f32 {
        match *self {
            PairPotential::LennardJones(lj) => {
                let term = (lj.sigma / r).powi(6);
                4.0 * lj.epsilon * (term * term - term)
            }
            PairPotential::Mie(mie) => {
                let term_a = (mie.sigma / r).powf(mie.gamma_r);
                let term_b = (mie.sigma / r).powf(mie.gamma_a);
                let c = (mie.gamma_r / (mie.gamma_r - mie.gamma_a))
                    * (mie.gamma_r / mie.gamma_a).powf(mie.gamma_a / (mie.gamma_r - mie.gamma_a));
                c * mie.epsilon * (term_a - term_b)
            }
            PairPotential::Morse(morse) => {
                let term_a = f32::exp(-2.0 * morse.a * (r - morse.r_e));
                let term_b = 2.0 * f32::exp(-morse.a * (r - morse.r_e));
                morse.d_e * (term_a - term_b)
            }
        }
    }
}

// TODO
// impl EnergyEvaluator for PairPotential {}
// impl ForceEvaluator for PairPotential {}
// impl EnergyForceEvaluator for PairPotential {}
