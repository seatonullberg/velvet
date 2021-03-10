//! Potentials which operate on pairs of atoms.

pub mod functions;

use serde::{Deserialize, Serialize};

use crate::internal::{self, Float};
use crate::neighbors::NeighborList;
use crate::potentials::Potential;
use crate::system::species::Specie;
use crate::system::System;

/// Shared behavior for pair potentials.
#[typetag::serde(tag = "type")]
pub trait PairPotential: Potential {
    /// Returns the potential energy of an atom in a pair separated by a distance `r`.
    fn energy(&self, r: Float) -> Float;
    /// Returns the magnitude of the force acting on an atom separated from another by a distance `r`.
    fn force(&self, r: Float) -> Float;
}

#[derive(Serialize, Deserialize)]
pub struct PairInteraction {
    pub potential: internal::Rc<dyn PairPotential>,
    pub cutoff: Float,
    pub index_i: usize,
    pub index_j: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PairPotentials {
    pub potentials: Vec<internal::Rc<dyn PairPotential>>,
    pub neighbor_lists: Vec<NeighborList>,
    pub interactions: Vec<PairInteraction>,
    pub update_frequency: usize,
}

impl PairPotentials {
    pub fn setup(&mut self, system: &System) {
        self.neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.setup(system));
    }

    pub fn update(&mut self, system: &System) {
        // update neighbor lists
        self.neighbor_lists.iter_mut().for_each(|nl| {
            nl.update(system)
        });
        // rebuild interactions
        self.interactions = self.potentials.iter().zip(self.neighbor_lists.iter()).fold(
            Vec::new(),
            |mut accumulator, (potential, nl)| {
                nl.indices().iter().for_each(|(i, j)| {
                    let interaction = PairInteraction {
                        potential: potential.clone(),
                        cutoff: nl.cutoff,
                        index_i: *i,
                        index_j: *j,
                    };
                    accumulator.push(interaction);
                });
                accumulator
            }
        )
    }
}

pub struct PairPotentialsBuilder {
    potentials: Vec<internal::Rc<dyn PairPotential>>,
    neighbor_lists: Vec<NeighborList>,
    interactions: Vec<PairInteraction>,
    update_frequency: usize,
}

impl PairPotentialsBuilder {
    pub fn new() -> PairPotentialsBuilder {
        PairPotentialsBuilder {
            potentials: Vec::new(),
            neighbor_lists: Vec::new(),
            interactions: Vec::new(),
            update_frequency: 1,
        }
    }

    pub fn add_pair(
        mut self,
        potential: Box<dyn PairPotential>,
        species: (Specie, Specie),
        cutoff: Float,
        thickness: Float,
    ) -> PairPotentialsBuilder {
        let potential = internal::Rc::from(potential);
        self.potentials.push(potential);
        let neighbor_list = NeighborList::new(cutoff, thickness, Some(species));
        self.neighbor_lists.push(neighbor_list);
        self
    }

    pub fn with_update_frequency(mut self, update_frequency: usize) -> PairPotentialsBuilder {
        self.update_frequency = update_frequency;
        self
    }

    pub fn build(self) -> PairPotentials {
        PairPotentials {
            potentials: self.potentials,
            neighbor_lists: self.neighbor_lists,
            interactions: self.interactions,
            update_frequency: self.update_frequency,
        }
    }
}

impl Default for PairPotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
