use crate::potential::Restriction;
use crate::system::element::Element;

/// Required behaviors for a pairwise interatomic potential.
pub trait PairPotential {
    /// Returns the potential energy of an atom separated from another by a distance `r`.
    fn energy(&self, r: f32) -> f32;
    /// Returns the magnitude of the force acting on an atom by another separated by a distance `r`.
    fn force(&self, r: f32) -> f32;
}

/// Pair potential meta data.
#[derive(Copy, Clone, Debug)]
pub struct PairMeta {
    /// Applicable elements.
    elements: (Element, Element),
    /// Limitation to the applicability.
    restriction: Restriction,
    /// Cutoff radius.
    cutoff: f32,
}
