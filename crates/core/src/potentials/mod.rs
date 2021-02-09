//! Classical interatomic potential functions.

pub mod pair;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::potentials::pair::{PairMeta, PairPotential};
use crate::system::System;

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    pair_data_map: HashMap<usize, (Box<dyn PairPotential>, PairMeta)>,
    pair_indices_map: HashMap<usize, Vec<(usize, usize)>>,
}

impl Potentials {
    pub fn initialize_pairs(&mut self, system: &System) {
        unimplemented!()
    }

    pub fn iter_pair_indices(&self, key: &usize) -> impl Iterator<Item = &(usize, usize)> {
        self.pair_indices_map.get(key).unwrap().iter()
    }
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    pair_data_map: HashMap<usize, (Box<dyn PairPotential>, PairMeta)>,
}

impl PotentialsBuilder {
    /// Returns a new `PotentialsBuilder`.
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            pair_data_map: HashMap::new(),
        }
    }

    /// Adds a new pair potential to the collection.
    ///
    /// # Arguments
    ///
    /// * `key` - ID associated with this pair type
    /// * `potential` - Boxed pair potential trait object
    /// * `meta` - Pair potential metadata
    pub fn add_pair(
        mut self,
        key: usize,
        potential: Box<dyn PairPotential>,
        meta: PairMeta,
    ) -> PotentialsBuilder {
        self.pair_data_map.insert(key, (potential, meta));
        self
    }

    /// Returns an initialized `Potentials`.
    pub fn build(self) -> Potentials {
        Potentials {
            pair_data_map: self.pair_data_map,
            pair_indices_map: HashMap::new(),
        }
    }
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
