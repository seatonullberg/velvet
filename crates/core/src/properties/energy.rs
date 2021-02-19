use serde::{Deserialize, Serialize};

use crate::potentials::Potentials;
use crate::properties::{IntrinsicProperty, Property};
use crate::system::System;

/// Potential energy due to coulombic potentials.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoulombEnergy;

impl Property for CoulombEnergy {
    type Res = f64;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let mut energy = 0 as f64;
        for (meta, potential) in &potentials.coulombs {
            for (i, j) in &meta.indices {
                let elem_i = system.elements[*i];
                let elem_j = system.elements[*j];
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell().distance(&pos_i, &pos_j);
                if meta.cutoff > r {
                    energy += potential.energy(elem_i.charge(), elem_j.charge(), r)
                        - potential.energy_self(elem_i.charge())
                        - potential.energy_self(elem_j.charge());
                }
            }
        }
        energy
    }
}

/// Potential energy due to pairwise potentials.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PairEnergy;

impl Property for PairEnergy {
    type Res = f64;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let mut energy = 0 as f64;
        for (meta, potential) in &potentials.pairs {
            for (i, j) in &meta.indices {
                let pos_i = system.positions[*i];
                let pos_j = system.positions[*j];
                let r = system.cell().distance(&pos_i, &pos_j);
                if meta.cutoff > r {
                    energy += potential.energy(r);
                }
            }
        }
        energy
    }
}

/// Potential energy of the whole system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Res = f64;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let coulomb_energy = CoulombEnergy.calculate(system, potentials);
        let pair_energy = PairEnergy.calculate(system, potentials);
        coulomb_energy + pair_energy
    }
}

/// Kinetic energy of the whole system
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KineticEnergy;

impl IntrinsicProperty for KineticEnergy {
    type Res = f64;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let kinetic_energy: f64 = system
            .elements
            .iter()
            .zip(system.velocities.iter())
            .map(|(elem, vel)| 0.5 * elem.mass() * vel.norm_squared())
            .sum();
        kinetic_energy
    }
}

/// Total energy of the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TotalEnergy;

impl Property for TotalEnergy {
    type Res = f64;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let kinetic = KineticEnergy.calculate(system, potentials);
        let potential = PotentialEnergy.calculate(system, potentials);
        kinetic + potential
    }
}
