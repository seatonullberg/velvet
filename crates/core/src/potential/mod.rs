pub mod pair;

use crate::potential::pair::{PairPotential, PairPotentialMeta};

pub trait Potential: Send + Sync {}

pub struct Potentials {
    pairs: Vec<(Box<dyn PairPotential>, PairPotentialMeta)>,
    // --bond--
    // --angle--
    // --dihedral--
    // --coulomb--
}

impl Potentials {
    pub fn new() -> Potentials {
        Potentials { pairs: Vec::new() }
    }

    pub fn pairs(&self) -> impl Iterator<Item = &(Box<dyn PairPotential>, PairPotentialMeta)> {
        self.pairs.iter()
    }

    pub fn add_pair(&mut self, potential: Box<dyn PairPotential>, meta: PairPotentialMeta) {
        self.pairs.push((potential, meta))
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
