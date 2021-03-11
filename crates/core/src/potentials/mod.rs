//! Classical interatomic potential functions.

pub mod coulomb;
pub mod pair;

use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::potentials::pair::{PairPotential, PairPotentials, PairPotentialsBuilder};
use crate::system::species::Specie;
use crate::system::System;

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
#[derive(Serialize, Deserialize)]
pub struct Potentials {
    pub pair_potentials: PairPotentials,
}

impl Potentials {
    pub fn setup(&mut self, system: &System) {
        // setup pair potentials
        self.pair_potentials.setup(system);
    }

    pub fn update(&mut self, system: &System, iteration: usize) {
        // update pair potentials
        if iteration % self.pair_potentials.update_frequency == 0 {
            self.pair_potentials.update(system);
        }
    }
}

/// Constructor for the [`Potentials`](velvet_core::potentials::Potentials) type.
pub struct PotentialsBuilder {
    pair_potentials_builder: PairPotentialsBuilder,
}

impl PotentialsBuilder {
    /// Returns a new `PotentialsBuilder`.
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            pair_potentials_builder: PairPotentialsBuilder::new(),
        }
    }

    pub fn with_pair_update_frequency(mut self, update_frequency: usize) -> PotentialsBuilder {
        self.pair_potentials_builder = self
            .pair_potentials_builder
            .with_update_frequency(update_frequency);
        self
    }

    pub fn add_pair(
        mut self,
        potential: Box<dyn PairPotential>,
        species: (Specie, Specie),
        cutoff: Float,
        thickness: Float,
    ) -> PotentialsBuilder {
        self.pair_potentials_builder = self
            .pair_potentials_builder
            .add_pair(potential, species, cutoff, thickness);
        self
    }

    pub fn build(self) -> Potentials {
        let pair_potentials = self.pair_potentials_builder.build();
        Potentials { pair_potentials }
    }
}

impl Default for PotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
