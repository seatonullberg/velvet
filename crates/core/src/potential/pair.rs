use crate::potential::{Potential, Restriction};
use crate::system::element::Element;

trait PairPotential {}

impl<T: Potential<Args = PairArgs>> PairPotential for T {}

#[derive(Copy, Clone, Debug)]
pub struct PairArgs {
    /// Distance between a pair of atoms.
    pub r: f32,
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
