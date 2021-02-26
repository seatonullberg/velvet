//! Classical interatomic potential functions.

pub mod coulomb;
pub mod pair;

use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::neighbors::{NeighborList, Neighbors, NeighborsBuilder};
use crate::potentials::pair::PairPotential;
use crate::system::species::Specie;
use crate::system::System;

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    neighbors: Neighbors,
    pairs: Vec<Box<dyn PairPotential>>,
    pub update_interval: usize,
}

impl Potentials {
    pub fn setup(&mut self, system: &System) {
        self.neighbors.setup(system);
    }

    pub fn update(&mut self, system: &System) {
        self.neighbors.update(system);
    }

    pub fn pair_interactions(&self) -> Vec<(&dyn PairPotential, usize, usize)> {
        self.neighbors
            .iter()
            .map(move |((i, j), index)| (&*self.pairs[*index], *i, *j))
            .collect()
    }
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    neighbors_builder: NeighborsBuilder,
    pairs: Vec<Box<dyn PairPotential>>,
    update_interval: usize,
}

impl PotentialsBuilder {
    /// Returns a new `PotentialsBuilder`.
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            neighbors_builder: NeighborsBuilder::new(),
            pairs: Vec::new(),
            update_interval: 1,
        }
    }

    pub fn with_pair(
        mut self,
        potential: Box<dyn PairPotential>,
        cutoff: Float,
        species: (Specie, Specie),
    ) -> PotentialsBuilder {
        let nl = NeighborList::new(cutoff, Some(species));
        self.neighbors_builder = self.neighbors_builder.with_neighbor_list(nl);
        self.pairs.push(potential);
        self
    }

    pub fn with_update_interval(mut self, interval: usize) -> PotentialsBuilder {
        self.update_interval = interval;
        self
    }

    pub fn build(self) -> Potentials {
        let neighbors = self.neighbors_builder.build();
        Potentials {
            neighbors,
            pairs: self.pairs,
            update_interval: self.update_interval,
        }
    }
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
