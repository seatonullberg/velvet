//! Types of energy that can be evaluated.

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::internal::Float;
use crate::potentials::Potentials;
use crate::potentials::coulomb::CoulombPotentialMeta;
use crate::potentials::pair::PairPotentialMeta;
use crate::properties::{IntrinsicProperty, Property};
use crate::system::System;

/// Potential energy due to Coulombic potentials.
#[derive(Clone, Copy, Debug)]
pub struct CoulombicEnergy;

impl CoulombicEnergy {
    fn calculate_inner(&self, meta: &CoulombPotentialMeta, system: &System, i: usize, j: usize) -> Float {
        let pos_i = system.positions[i];
        let qi = system.species[i].charge();
        let pos_j = system.positions[j];
        let qj = system.species[j].charge();
        let r = system.cell.distance(&pos_i, &pos_j);
        if r < meta.cutoff {
            meta.potential.energy(qi, qj, r)
        } else {
            0.0
        }
    }
}

impl Property for CoulombicEnergy {
    type Res = Float;

    #[cfg(not(feature = "rayon"))]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        match &potentials.coulomb_meta {
            None => 0.0,
            Some(meta) => meta
                .selection
                .indices()
                .map(|&[i, j]| {
                    self.calculate_inner(meta, system, i, j)
                }).sum()
        }
    }

    #[cfg(feature = "rayon")]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        match &potentials.coulomb_meta {
            None => 0.0,
            Some(meta) => meta
                .selection
                .par_indices()
                .map(|&[i, j]| {
                    self.calculate_inner(meta, system, i, j)
                }).sum()
        }
    }

    fn name(&self) -> String {
        "coulombic_energy".to_string()
    }
}

/// Potential energy due to pairwise potentials.
#[derive(Clone, Copy, Debug)]
pub struct PairEnergy;

impl PairEnergy {
    fn calculate_inner(&self, meta: &PairPotentialMeta, system: &System, i: usize, j: usize) -> Float {
        let pos_i = system.positions[i];
        let pos_j = system.positions[j];
        let r = system.cell.distance(&pos_i, &pos_j);
        if r < meta.cutoff {
            meta.potential.energy(r)
        } else {
            0.0
        }
    }
}

impl Property for PairEnergy {
    type Res = Float;

    #[cfg(not(feature = "rayon"))]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pair_metas
            .iter()
            .map(|meta| -> Float {
                meta.selection
                    .indices()
                    .map(|&[i, j]| -> Float {
                        self.calculate_inner(meta, system, i, j)
                    }).sum()
            }).sum()
    }

    #[cfg(feature = "rayon")]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pair_metas
            .iter()
            .map(|meta| -> Float {
                meta.selection
                    .par_indices()
                    .map(|&[i, j]| -> Float {
                        self.calculate_inner(meta, system, i, j)
                    }).sum()
            }).sum()
    }

    fn name(&self) -> String {
        "pair_energy".to_string()
    }
}

/// Potential energy of the whole system.
#[derive(Clone, Copy, Debug)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Res = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let coulomb_energy = CoulombicEnergy.calculate(system, potentials);
        let pair_energy = PairEnergy.calculate(system, potentials);
        coulomb_energy + pair_energy
    }

    fn name(&self) -> String {
        "potential_energy".to_string()
    }
}

/// Kinetic energy of the whole system
#[derive(Clone, Copy, Debug)]
pub struct KineticEnergy;

impl IntrinsicProperty for KineticEnergy {
    type Res = Float;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let kinetic_energy: Float = system
            .species
            .iter()
            .zip(system.velocities.iter())
            .map(|(species, vel)| {
                0.5 * species.mass() * vel.norm_squared()
            })
            .sum();
        kinetic_energy
    }

    fn name(&self) -> String {
        "kinetic_energy".to_string()
    }
}

/// Sum of potential and kinetic energy.
#[derive(Clone, Copy, Debug)]
pub struct TotalEnergy;

impl Property for TotalEnergy {
    type Res = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let kinetic = KineticEnergy.calculate(system, potentials);
        let potential = PotentialEnergy.calculate(system, potentials);
        kinetic + potential
    }

    fn name(&self) -> String {
        "total_energy".to_string()
    }
}
