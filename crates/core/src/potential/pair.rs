use crate::potential::{Potential, Restriction};
use crate::system::element::Element;

pub trait PairPotential: Potential {
    fn energy(&self, r: f32) -> f32;
    fn force(&self, r: f32) -> f32;
}

/// Pair potential meta data.
#[derive(Clone, Copy, Debug)]
pub struct PairPotentialMeta {
    /// Applicable elements.
    pub elements: (Element, Element),
    /// Cutoff radius.
    pub cutoff: f32,
    /// Limitation to the applicability.
    pub restriction: Restriction,
}

impl PairPotentialMeta {
    pub fn new(
        elements: (Element, Element),
        cutoff: f32,
        restriction: Restriction,
    ) -> PairPotentialMeta {
        PairPotentialMeta {
            elements,
            cutoff,
            restriction,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LennardJones {
    epsilon: f32,
    sigma: f32,
}

impl LennardJones {
    pub fn new(epsilon: f32, sigma: f32) -> LennardJones {
        LennardJones { epsilon, sigma }
    }
}

impl Potential for LennardJones {}

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

#[derive(Clone, Copy, Debug)]
pub struct Harmonic {
    k: f32,
    x0: f32,
}

impl Harmonic {
    pub fn new(k: f32, x0: f32) -> Harmonic {
        Harmonic { k, x0 }
    }
}

impl Potential for Harmonic {}

impl PairPotential for Harmonic {
    fn energy(&self, r: f32) -> f32 {
        let dr = r - self.x0;
        0.5 * self.k * dr * dr
    }

    fn force(&self, r: f32) -> f32 {
        self.k * (self.x0 - r)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mie {
    epsilon: f32,
    sigma: f32,
    gamma_a: f32,
    gamma_r: f32,
}

impl Mie {
    pub fn new(epsilon: f32, sigma: f32, gamma_a: f32, gamma_r: f32) -> Mie {
        Mie {
            epsilon,
            sigma,
            gamma_a,
            gamma_r,
        }
    }
}

impl Potential for Mie {}

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

#[derive(Clone, Copy, Debug)]
pub struct Morse {
    a: f32,
    d_e: f32,
    r_e: f32,
}

impl Morse {
    pub fn new(a: f32, d_e: f32, r_e: f32) -> Morse {
        Morse { a, d_e, r_e }
    }
}

impl Potential for Morse {}

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
    use crate::potential::pair::{Harmonic, LennardJones, Mie, Morse, PairPotential};
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
