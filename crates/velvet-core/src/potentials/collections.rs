//! Collections of interatomic potentials grouped by interaction type.

use crate::internal::Float;
use crate::neighbors::NeighborList;
use crate::potentials::pair::PairPotential;
use crate::system::particle::ParticleType;
use crate::system::System;

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
    /// * `update_frequency` - Number of iterations to complete between updates.
    pub fn with_pair_update_frequency(mut self, update_frequency: usize) -> PotentialsBuilder {
        self.pair_potentials_builder = self
            .pair_potentials_builder
            .with_update_frequency(update_frequency);
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
    pub fn add_pair<P>(
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
                .add_pair(potential, particle_types, cutoff, thickness);
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

pub(crate) struct PairPotentials {
    pub potentials: Vec<Box<dyn PairPotential>>,
    pub neighbor_lists: Vec<NeighborList>,
    pub cutoffs: Vec<Float>,
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
        self.neighbor_lists
            .iter_mut()
            .for_each(|nl| nl.update(system));
    }
}

/// Convenient constructor for [`PairPotentials`].
pub(crate) struct PairPotentialsBuilder {
    potentials: Vec<Box<dyn PairPotential>>,
    neighbor_lists: Vec<NeighborList>,
    cutoffs: Vec<Float>,
    update_frequency: usize,
}

impl PairPotentialsBuilder {
    /// Returns a new `PairPotentialsBuilder`.
    pub fn new() -> PairPotentialsBuilder {
        PairPotentialsBuilder {
            potentials: Vec::new(),
            neighbor_lists: Vec::new(),
            cutoffs: Vec::new(),
            update_frequency: 1,
        }
    }

    /// Adds a new potential to the collection.
    pub fn add_pair<P>(
        mut self,
        potential: P,
        particle_types: (ParticleType, ParticleType),
        cutoff: Float,
        thickness: Float,
    ) -> PairPotentialsBuilder
    where
        P: PairPotential + 'static,
    {
        let potential = Box::new(potential);
        self.potentials.push(potential);
        let neighbor_list = NeighborList::new(cutoff + thickness, Some(particle_types));
        self.neighbor_lists.push(neighbor_list);
        self.cutoffs.push(cutoff);
        self
    }

    /// Sets the number of iterations between each call to `update`.
    pub fn with_update_frequency(mut self, update_frequency: usize) -> PairPotentialsBuilder {
        self.update_frequency = update_frequency;
        self
    }

    /// Consumes the builder and returns a new [`PairPotentials`] object.
    pub fn build(self) -> PairPotentials {
        PairPotentials {
            potentials: self.potentials,
            neighbor_lists: self.neighbor_lists,
            cutoffs: self.cutoffs,
            update_frequency: self.update_frequency,
        }
    }
}

impl Default for PairPotentialsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
