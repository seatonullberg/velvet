//! Interatomic potentials for evaluating potential energy and forces.

pub mod pair;

use serde::{Deserialize, Serialize};

use crate::potential::pair::{PairPotential, PairPotentialMeta};

/// Base trait for all potentials.
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    pairs: Vec<(Box<dyn PairPotential>, PairPotentialMeta)>,
    // --bond--
    // --angle--
    // --dihedral--
    // --coulomb--
}

impl Potentials {
    /// Returns a new `Potentials`.
    pub fn new() -> Potentials {
        Potentials { pairs: Vec::new() }
    }

    /// Returns an iterator over each pair potential.
    pub fn pairs(&self) -> impl Iterator<Item = &(Box<dyn PairPotential>, PairPotentialMeta)> {
        self.pairs.iter()
    }

    /// Adds a new pair potentials to the collection.
    pub fn add_pair(&mut self, potential: Box<dyn PairPotential>, meta: PairPotentialMeta) {
        self.pairs.push((potential, meta))
    }
}

/// Restrictions which can be applied to any potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Restriction {
    /// No restrictions.
    None,
    /// Applies only to atoms in separate molecules.
    Intermolecular,
    /// Applies only to atoms within the same molecule.
    Intramolecular,
}
