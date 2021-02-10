//! Classical interatomic potential functions.

pub mod pair;

use serde::{Deserialize, Serialize};

use crate::potentials::pair::PairDescriptor;

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    pub pairs: Vec<PairDescriptor>,
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    pairs: Vec<PairDescriptor>,
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
    /// * `pair` - PairDescriptor to add to the set of potentials
    pub fn add_pair(mut self, pair: PairDescriptor) -> PotentialsBuilder {
        self.pairs.push(pair);
        self
    }

    /// Returns an initialized `Potentials`.
    pub fn build(self) -> Potentials {
        Potentials { pairs: self.pairs }
    }
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
