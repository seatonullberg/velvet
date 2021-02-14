//! Classical interatomic potential functions.

pub mod coulomb;
pub mod pair;

use serde::{Deserialize, Serialize};

use crate::potentials::coulomb::{CoulombMeta, CoulombPotential};
use crate::potentials::pair::{PairMeta, PairPotential};

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    pub coulombs: Vec<(CoulombMeta, Box<dyn CoulombPotential>)>,
    pub pairs: Vec<(PairMeta, Box<dyn PairPotential>)>,
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    coulombs: Vec<(CoulombMeta, Box<dyn CoulombPotential>)>,
    pairs: Vec<(PairMeta, Box<dyn PairPotential>)>,
}

impl PotentialsBuilder {
    /// Returns a new `PotentialsBuilder`.
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            coulombs: Vec::new(),
            pairs: Vec::new(),
        }
    }

    /// Adds a new coulomb potential to the collection.
    ///
    /// # Arguments
    ///
    /// * `meta` - Coulomb metadata.
    /// * `potential` - Boxed coulomb potential.
    pub fn add_coulomb(
        mut self,
        meta: CoulombMeta,
        potential: Box<dyn CoulombPotential>,
    ) -> PotentialsBuilder {
        self.coulombs.push((meta, potential));
        self
    }

    /// Adds a new pair potential to the collection.
    ///
    /// # Arguments
    ///
    /// * `meta` - Pairwise metadata.
    /// * `potential` - Boxed pair potential.
    pub fn add_pair(
        mut self,
        meta: PairMeta,
        potential: Box<dyn PairPotential>,
    ) -> PotentialsBuilder {
        self.pairs.push((meta, potential));
        self
    }

    /// Returns an initialized `Potentials`.
    pub fn build(self) -> Potentials {
        Potentials {
            coulombs: self.coulombs,
            pairs: self.pairs,
        }
    }
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
