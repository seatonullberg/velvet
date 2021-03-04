//! Classical interatomic potential functions.

pub mod coulomb;
pub mod pair;

use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::neighbors::NeighborList;
use crate::potentials::pair::{PairInteraction, PairPotential};
use crate::system::System;

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    pair_potentials: Vec<Rc<dyn PairPotential>>,
    pair_neighbor_lists: Vec<NeighborList>,
    pair_interactions: Vec<PairInteraction>,
}

impl Potentials {
    pub fn setup(&mut self, system: &System) {
        // setup pair potentials
        self.pair_neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.setup(system));

        // force an initial update
        self.update(system, 0);
    }

    pub fn update(&mut self, system: &System, iteration: usize) {
        // update pair potential neighbor lists
        self.pair_neighbor_lists.iter_mut().for_each(|nl| {
            if iteration % nl.update_frequency == 0 {
                nl.update(system)
            }
        });
        // rebuild the pair potential interactions
        self.pair_interactions = self
            .pair_potentials
            .iter()
            .zip(self.pair_neighbor_lists.iter())
            .fold(Vec::new(), |mut accumulator, (potential, nl)| {
                nl.indices().iter().for_each(|(i, j)| {
                    let interaction = PairInteraction {
                        potential: potential.clone(),
                        index_i: *i,
                        index_j: *j,
                    };
                    accumulator.push(interaction);
                });
                accumulator
            });
    }

    pub fn pair_interactions(&self) -> &Vec<PairInteraction> {
        &self.pair_interactions
    }
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    pair_potentials: Vec<Rc<dyn PairPotential>>,
    pair_neighbor_lists: Vec<NeighborList>,
    pair_interactions: Vec<PairInteraction>,
}

impl PotentialsBuilder {
    /// Returns a new `PotentialsBuilder`.
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            pair_potentials: Vec::new(),
            pair_neighbor_lists: Vec::new(),
            pair_interactions: Vec::new(),
        }
    }

    pub fn with_pair(
        mut self,
        potential: Rc<dyn PairPotential>,
        neighbor_list: NeighborList,
    ) -> PotentialsBuilder {
        self.pair_potentials.push(potential);
        self.pair_neighbor_lists.push(neighbor_list);
        self
    }

    pub fn build(self) -> Potentials {
        Potentials {
            pair_potentials: self.pair_potentials,
            pair_neighbor_lists: self.pair_neighbor_lists,
            pair_interactions: self.pair_interactions,
        }
    }
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
