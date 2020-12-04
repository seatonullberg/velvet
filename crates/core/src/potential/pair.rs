use crate::potential::{Potential, Restriction};
use crate::system::element::Element;

pub trait PairPotential: Potential {
    fn energy(&self, r: f32) -> f32;
    fn force(&self, r: f32) -> f32;
}

/// Pair potential meta data.
#[derive(Copy, Clone, Debug)]
pub struct PairMeta {
    /// Applicable elements.
    pub elements: (Element, Element),
    /// Limitation to the applicability.
    pub restriction: Restriction,
    /// Cutoff radius.
    pub cutoff: f32,
}
