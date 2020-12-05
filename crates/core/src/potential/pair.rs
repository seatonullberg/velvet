use crate::potential::{Potential, Restriction};
use crate::system::element::Element;

pub trait PairPotential: Potential {
    fn energy(&self, r: f32) -> f32;
    fn force(&self, r: f32) -> f32;
}

#[derive(Clone, Copy, Debug)]
pub enum PairPotentialEnum {
    LennardJones(LennardJones),
}

impl Potential for PairPotentialEnum {}

impl PairPotential for PairPotentialEnum {
    fn energy(&self, r: f32) -> f32 {
        match self {
            PairPotentialEnum::LennardJones(lj) => lj.energy(r),
        }
    }

    fn force(&self, r: f32) -> f32 {
        match self {
            PairPotentialEnum::LennardJones(lj) => lj.force(r),
        }
    }
}

/// Pair potential meta data.
#[derive(Clone, Copy, Debug)]
pub struct PairPotentialMeta {
    /// Applicable elements.
    pub elements: (Element, Element),
    /// Limitation to the applicability.
    pub restriction: Restriction,
    /// Cutoff radius.
    pub cutoff: f32,
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

#[cfg(test)]
mod tests {
    use crate::potential::pair::{LennardJones, PairPotential};

    #[test]
    fn lennard_jones() {
        let lj = LennardJones::new(0.8, 2.0);
        assert_eq!(lj.energy(2.0), 0.0);
        assert_eq!(lj.energy(2.5), -0.6189586);
        assert!(lj.force(f32::powf(2.0, 1.0 / 6.0) * 2.0).abs() < 1e-6);
        assert_eq!(lj.force(2.5), -0.9577348);
    }
}
