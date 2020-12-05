pub mod pair;

use crate::potential::pair::{PairPotentialEnum, PairPotentialMeta};

pub trait Potential: Clone + Copy + Send + Sync {}

pub struct Potentials {
    pair: Vec<(PairPotentialEnum, PairPotentialMeta)>,
    // --bond--
    // --angle--
    // --dihedral--
    // --coulomb--
}

impl Potentials {
    pub fn pairs(&self) -> impl Iterator<Item = &(PairPotentialEnum, PairPotentialMeta)> {
        self.pair.iter()
    }
}

/// Restrictions which can be applied to a potential.
#[derive(Clone, Copy, Debug)]
pub enum Restriction {
    /// No restrictions.
    None,
    /// Applies only to atoms in separate molecules.
    Intermolecular,
    /// Applies only to atoms within the same molecule.
    Intramolecular,
}
