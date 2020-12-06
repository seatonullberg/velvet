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

#[cfg(test)]
mod tests {
    use crate::potential::pair::{Harmonic, LennardJones, PairPotential};
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
}
