//! Classical interatomic potentials.

pub mod coulomb;
pub mod functions;
pub mod pair;

use crate::internal::Float;
use crate::potentials::coulomb::{CoulombPotential, CoulombPotentialMeta};
use crate::potentials::pair::{PairPotential, PairPotentialMeta};
use crate::system::particle::ParticleType;
use crate::system::System;

/// Base trait for all potentials.
pub trait Potential: Send + Sync {}

pub struct Potentials {
    pub(crate) coulomb_meta: Option<CoulombPotentialMeta>,
    pub(crate) pair_metas: Vec<PairPotentialMeta>,
    pub(crate) update_frequency: usize,
}

impl Potentials {
    pub fn setup(&mut self, system: &System) {
        // setup coulomb potential if it exists
        match &mut self.coulomb_meta {
            Some(meta) => meta.setup(system),
            None => {}
        }
        // setup each pair potential
        self.pair_metas
            .iter_mut()
            .for_each(|meta| meta.setup(system))
    }

    pub fn update(&mut self, system: &System, iteration: usize) {
        // only update if the update frequency is reached
        if iteration % self.update_frequency != 0 {
            return;
        }
        // update coulomb potential if it exists
        match &mut self.coulomb_meta {
            Some(meta) => meta.update(system),
            None => {}
        }
        // update each pair potential
        self.pair_metas
            .iter_mut()
            .for_each(|meta| meta.update(system))
    }
}

pub struct PotentialsBuilder {
    coulomb_meta: Option<CoulombPotentialMeta>,
    pair_metas: Vec<PairPotentialMeta>,
    update_frequency: usize,
}

impl PotentialsBuilder {
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            coulomb_meta: None,
            pair_metas: Vec::new(),
            update_frequency: 1,
        }
    }

    pub fn coulomb<T>(mut self, potential: T, cutoff: Float, thickness: Float) -> PotentialsBuilder
    where
        T: CoulombPotential + 'static,
    {
        self.coulomb_meta = Some(CoulombPotentialMeta::new(potential, cutoff, thickness));
        self
    }

    pub fn pair<T>(
        mut self,
        potential: T,
        particle_types: (ParticleType, ParticleType),
        cutoff: Float,
        thickness: Float,
    ) -> PotentialsBuilder
    where
        T: PairPotential + 'static,
    {
        self.pair_metas.push(PairPotentialMeta::new(
            potential,
            particle_types,
            cutoff,
            thickness,
        ));
        self
    }

    pub fn update_frequency(mut self, freq: usize) -> PotentialsBuilder {
        self.update_frequency = freq;
        self
    }

    pub fn build(self) -> Potentials {
        Potentials {
            coulomb_meta: self.coulomb_meta,
            pair_metas: self.pair_metas,
            update_frequency: self.update_frequency,
        }
    }
}

// /// Container type to hold instances of each potential in the system.
// pub struct Potentials {
//     pub(crate) coulomb_potentials: CoulombPotentials,
//     pub(crate) pair_potentials: PairPotentials,
// }

// impl Potentials {
//     pub(crate) fn setup(&mut self, system: &System) {
//         self.coulomb_potentials.setup(system);
//         self.pair_potentials.setup(system);
//     }

//     pub(crate) fn update(&mut self, system: &System, iteration: usize) {
//         if iteration % self.coulomb_potentials.update_frequency == 0 {
//             self.coulomb_potentials.update(system);
//         }
//         if iteration % self.pair_potentials.update_frequency == 0 {
//             self.pair_potentials.update(system);
//         }
//     }
// }

// /// Convenient constructor for [`Potentials`].
// pub struct PotentialsBuilder {
//     coulomb_potentials_builder: CoulombPotentialsBuilder,
//     pair_potentials_builder: PairPotentialsBuilder,
// }

// impl PotentialsBuilder {
//     /// Returns a new `PotentialsBuilder`.
//     pub fn new() -> PotentialsBuilder {
//         PotentialsBuilder {
//             coulomb_potentials_builder: CoulombPotentialsBuilder::new(),
//             pair_potentials_builder: PairPotentialsBuilder::new(),
//         }
//     }

//     /// Sets the `update_frequency` field of the underlying [`CoulombPotentials`] object.
//     ///
//     /// # Arguments
//     ///
//     /// * `freq` - Number of iterations between updates.
//     pub fn coulomb_update_frequency(mut self, freq: usize) -> PotentialsBuilder {
//         self.coulomb_potentials_builder = self.coulomb_potentials_builder.update_frequency(freq);
//         self
//     }

//     /// Sets the `update_frequency` field of the underlying [`PairPotentials`] object.
//     ///
//     /// # Arguments
//     ///
//     /// * `freq` - Number of iterations between updates.
//     pub fn pair_update_frequency(mut self, freq: usize) -> PotentialsBuilder {
//         self.pair_potentials_builder = self.pair_potentials_builder.update_frequency(freq);
//         self
//     }

//     /// Adds a new coulomb potential to the collection.
//     ///
//     /// # Arguments
//     ///
//     /// * `potential` - [`CoulombPotential`] trait object.
//     /// * `cutoff` - Cutoff radius.
//     /// * `thickness` - Buffer thickness used to construct a neighbor list.
//     pub fn coulomb<P>(mut self, potential: P, cutoff: Float, thickness: Float) -> PotentialsBuilder
//     where
//         P: CoulombPotential + 'static,
//     {
//         self.coulomb_potentials_builder = self
//             .coulomb_potentials_builder
//             .coulomb(potential, cutoff, thickness);
//         self
//     }

//     /// Adds a new pair potential to the collection.
//     ///
//     /// # Arguments
//     ///
//     /// * `potential` - [`PairPotential`] trait object.
//     /// * `particle_types` - Tuple of [`ParticleTypes`] objects that the potential applies to.
//     /// * `cutoff` - Cutoff radius.
//     /// * `thickness` - Buffer thickness used to construct a neighbor list.
//     pub fn pair<P>(
//         mut self,
//         potential: P,
//         particle_types: (ParticleType, ParticleType),
//         cutoff: Float,
//         thickness: Float,
//     ) -> PotentialsBuilder
//     where
//         P: PairPotential + 'static,
//     {
//         self.pair_potentials_builder =
//             self.pair_potentials_builder
//                 .pair(potential, particle_types, cutoff, thickness);
//         self
//     }

//     /// Consumes the builder and returns a new [`Potentials`] object.
//     pub fn build(self) -> Potentials {
//         let coulomb_potentials = self.coulomb_potentials_builder.build();
//         let pair_potentials = self.pair_potentials_builder.build();
//         Potentials { coulomb_potentials, pair_potentials }
//     }
// }

// impl Default for PotentialsBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }
