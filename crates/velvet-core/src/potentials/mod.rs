//! Classical interatomic potentials.

pub mod coulomb;
pub mod functions;
pub mod pair;

use crate::internal::Float;
use crate::potentials::pair::{PairPotential, PairPotentials, PairPotentialsBuilder};
use crate::system::particle::ParticleType;
use crate::system::System;

/// Base trait for all potentials.
pub trait Potential: Send + Sync {}

/// Container type to hold instances of each potential in the system.
pub struct Potentials {
    pub(crate) pair_potentials: PairPotentials,
}

impl Potentials {
    pub(crate) fn setup(&mut self, system: &System) {
        self.pair_potentials.setup(system);
    }

    pub(crate) fn update(&mut self, system: &System, iteration: usize) {
        if iteration % self.pair_potentials.update_frequency == 0 {
            self.pair_potentials.update(system);
        }
    }
}

/// Convenient constructor for [`Potentials`].
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

    /// Sets the `update_frequency` field of the underlying [`PairPotentials`] object.
    ///
    /// # Arguments
    ///
    /// * `freq` - Number of iterations between updates.
    pub fn pair_update_frequency(mut self, freq: usize) -> PotentialsBuilder {
        self.pair_potentials_builder = self.pair_potentials_builder.update_frequency(freq);
        self
    }

    /// Adds a new pair potential to the collection.
    ///
    /// # Arguments
    ///
    /// * `potential` - [`PairPotential`] trait object.
    /// * `particle_types` - Tuple of [`ParticleTypes`] objects that the potential applies to.
    /// * `cutoff` - Cutoff radius.
    /// * `thickness` - Buffer thickness used to construct a [`NeighborList`].
    pub fn pair<P>(
        mut self,
        potential: P,
        particle_types: (ParticleType, ParticleType),
        cutoff: Float,
        thickness: Float,
    ) -> PotentialsBuilder
    where
        P: PairPotential + 'static,
    {
        self.pair_potentials_builder =
            self.pair_potentials_builder
                .pair(potential, particle_types, cutoff, thickness);
        self
    }

    /// Consumes the builder and returns a new [`Potentials`] object.
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
