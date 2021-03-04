use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::potentials::Potentials;
use crate::properties::{IntrinsicProperty, Property};
use crate::system::System;

/// Potential energy due to pairwise potentials.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PairEnergy;

impl Property for PairEnergy {
    type Res = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pair_interactions()
            .iter()
            .map(|interaction| {
                let potential = &interaction.potential;
                let i = interaction.index_i;
                let j = interaction.index_j;
                let pos_i = system.positions[i];
                let pos_j = system.positions[j];
                let r = system.cell.distance(&pos_i, &pos_j);
                potential.energy(r)
            })
            .sum()
    }
}

/// Potential energy of the whole system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Res = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pair_energy = PairEnergy.calculate(system, potentials);
        pair_energy
    }
}

/// Kinetic energy of the whole system
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KineticEnergy;

impl IntrinsicProperty for KineticEnergy {
    type Res = Float;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let kinetic_energy: Float = system
            .specie_ids
            .iter()
            .zip(system.velocities.iter())
            .map(|(id, vel)| {
                let sp = system.species[id];
                0.5 * sp.mass() * vel.norm_squared()
            })
            .sum();
        kinetic_energy
    }
}

/// Total energy of the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TotalEnergy;

impl Property for TotalEnergy {
    type Res = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let kinetic = KineticEnergy.calculate(system, potentials);
        let potential = PotentialEnergy.calculate(system, potentials);
        kinetic + potential
    }
}
