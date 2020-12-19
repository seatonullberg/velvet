//! Classical interatomic potential functions.

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

impl Potentials {
    /// Returns an iterator over each pair potential in the collection.
    pub fn pairs(&self) -> impl Iterator<Item = &(Box<dyn PairPotential>, PairPotentialMeta)> {
        self.pairs.iter()
    }
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    pairs: Vec<(Box<dyn PairPotential>, PairPotentialMeta)>,
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PotentialsBuilder {
    /// Returns a new `PotentialsBuilder`.
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder { pairs: Vec::new() }
    }

    /// Adds a new pair potential to the collection.
    ///
    /// # Arguments
    ///
    /// * `potential` - Boxed pair potential trait object
    /// * `meta` - Pair potential metadata
    pub fn with_pair(
        mut self,
        potential: Box<dyn PairPotential>,
        meta: PairPotentialMeta,
    ) -> PotentialsBuilder {
        self.pairs.push((potential, meta));
        self
    }

    /// Returns an initialized `Potentials`.
    pub fn finish(self) -> Potentials {
        Potentials { pairs: self.pairs }
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
