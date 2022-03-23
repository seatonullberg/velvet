//! Interatomic potential functions.

pub mod pair;
pub mod types;

use crate::potentials::pair::{PairMeta, PairPotential};
use velvet_internals::float::Float;
use velvet_system::species::Species;
use velvet_system::System;

pub struct Potentials {
    pub pair_metas: Option<Vec<PairMeta>>,
}

impl Potentials {
    pub fn setup(&mut self, system: &System) {
        match &mut self.pair_metas {
            Some(pair_metas) => pair_metas.iter_mut().for_each(|meta| meta.setup(system)),
            None => {}
        }
    }

    pub fn update(&mut self, system: &System) {
        match &mut self.pair_metas {
            Some(pair_metas) => pair_metas.iter_mut().for_each(|meta| meta.update(system)),
            None => {}
        }
    }
}

pub struct PotentialsBuilder {
    pair_metas: Option<Vec<PairMeta>>,
}

impl PotentialsBuilder {
    pub fn new() -> Self {
        PotentialsBuilder { pair_metas: None }
    }

    pub fn pair<P>(mut self, potential: P, species_i: Species, species_j: Species, cutoff: Float) -> Self
    where
        P: PairPotential + 'static,
    {
        let pair_meta = PairMeta::new(potential, species_i, species_j, cutoff);
        let pair_metas = &mut self.pair_metas;
        match pair_metas {
            Some(pair_metas) => {
                pair_metas.push(pair_meta);
            }
            None => {
                let mut pair_metas = Vec::new();
                pair_metas.push(pair_meta);
                self.pair_metas = Some(pair_metas);
            }
        }
        self
    }

    pub fn build(self) -> Potentials {
        let pair_metas = self.pair_metas;
        Potentials { pair_metas }
    }
}
