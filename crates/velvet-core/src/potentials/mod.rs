//! Classical interatomic potentials.

pub mod coulomb;
pub mod pair;
pub mod types;

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
