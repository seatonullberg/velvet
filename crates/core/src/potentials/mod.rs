//! Interatomic potentials to evaluate potential energy and forces.

pub mod pair;

use serde::{Deserialize, Serialize};

use crate::potentials::pair::{PairPotential, PairPotentialMeta};

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
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

impl Default for Potentials {
    fn default() -> Self {
        Self::new()
    }
}

impl Potentials {
    /// Returns an empty collection of potentials.
    pub fn new() -> Potentials {
        Potentials { pairs: Vec::new() }
    }

    /// Returns an iterator over each pair potential.
    pub fn pairs(&self) -> impl Iterator<Item = &(Box<dyn PairPotential>, PairPotentialMeta)> {
        self.pairs.iter()
    }

    /// Adds a new pair potential to the collection.
    ///
    /// # Arguments
    ///
    /// * `potential` - Boxed pair potential trait object
    /// * `meta` - Pair potential metadata
    pub fn add_pair(&mut self, potential: Box<dyn PairPotential>, meta: PairPotentialMeta) {
        self.pairs.push((potential, meta))
    }
}

/// Restrictions on the applicability of a potential.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Restriction {
    /// No restrictions.
    None,
    /// Applies only to atoms in separate molecules.
    Intermolecular,
    /// Applies only to atoms within the same molecule.
    Intramolecular,
}
