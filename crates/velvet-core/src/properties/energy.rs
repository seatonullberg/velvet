//! Types of energy that can be evaluated.

use nalgebra::Vector3;
use rayon::prelude::*;

use crate::internal::Float;
use crate::potentials::pair::PairPotentialMeta;
use crate::potentials::Potentials;
use crate::properties::{IntrinsicProperty, Property};
use crate::system::System;

/// Potential energy due to pairwise potentials.
#[derive(Clone, Copy, Debug)]
pub struct PairEnergy;

impl PairEnergy {
    fn calculate_inner(
        &self,
        meta: &PairPotentialMeta,
        system: &System,
        indices: &[[usize; 2]],
    ) -> Float {
        // Initialize loop variables.
        let mut total: Float = 0 as Float;
        let mut pos_i: Vector3<Float> = Vector3::zeros();
        let mut pos_j: Vector3<Float> = Vector3::zeros();
        let mut r: Float = 0 as Float;
        // Iterate over the pairs of indices and sum the energy contribution of each one.
        indices.iter().for_each(|&[i, j]| {
            pos_i = system.positions[i];
            pos_j = system.positions[j];
            r = system.cell.distance(&pos_i, &pos_j);
            if r < meta.cutoff {
                total += meta.potential.energy(r)
            }
        });
        // Return the total energy of the given pairs.
        total
    }
}

impl Property for PairEnergy {
    type Res = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pair_metas
            .iter()
            .map(|meta| -> Float {
                meta.selection
                    .par_iter_chunks()
                    .map(|chunk| -> Float { self.calculate_inner(meta, system, chunk) })
                    .sum()
            })
            .sum()
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
        let pair_energy = PairEnergy.calculate(system, potentials);
        pair_energy
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
            .map(|(species, vel)| 0.5 * species.mass() * vel.norm_squared())
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
