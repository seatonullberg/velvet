//! Interatomic potential functions.

pub mod pair;
pub mod types;

use crate::potentials::pair::{PairMeta, PairPotential};
use velvet_internals::float::Float;
use velvet_system::species::Species;
use velvet_system::System;

pub struct Potentials {
    pub pair_metas: Option<Vec<PairMeta>>,
    // pub coulomb_meta: Option<CoulombMeta>,
    // pub bond_metas: Option<Vec<BondMeta>>,
    // pub angle_metas: Option<Vec<AngleMeta>>,
    // pub dihedral_metas: Option<Vec<DihedralMeta>>,
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
    // pub coulomb_meta: Option<CoulombMeta>,
    // pub bond_metas: Option<Vec<BondMeta>>,
    // pub angle_metas: Option<Vec<AngleMeta>>,
    // pub dihedral_metas: Option<Vec<DihedralMeta>>,
}

impl PotentialsBuilder {
    pub fn new() -> Self {
        PotentialsBuilder { pair_metas: None }
    }

    pub fn pair<P>(mut self, species: (Species, Species), cutoff: Float, potential: P) -> Self
    where
        P: PairPotential + 'static,
    {
        let pair_meta = PairMeta::new(species, cutoff, potential);
        let pair_metas = &mut self.pair_metas;
        match pair_metas {
            Some(pair_metas) => {
                pair_metas.push(pair_meta);
            }
            None => {
                let mut pair_metas = Vec::with_capacity(1);
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
