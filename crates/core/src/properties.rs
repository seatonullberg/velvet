//! Physical properties of the simulated system.

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::constants::BOLTZMANN;
use crate::potentials::Potentials;
use crate::system::System;

/// Calculates a system-wide property.
pub trait Property {
    /// The property's return type.
    type Res;
    /// Returns a physical property of the system.
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res;
}

/// Calculates a system-wide property without using the applied potentials.
pub trait IntrinsicProperty {
    /// The property's return type.
    type Res;
    /// Returns a physical property of the system without accessing the associated potentials.
    fn calculate_intrinsic(&self, system: &System) -> Self::Res;
}

impl<T: IntrinsicProperty> Property for T {
    type Res = T::Res;

    fn calculate(&self, system: &System, _: &Potentials) -> Self::Res {
        <T as IntrinsicProperty>::calculate_intrinsic(&self, system)
    }
}

/// Force acting on each atom in the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Forces;

impl Property for Forces {
    type Res = Vec<Vector3<f32>>;

    #[cfg(not(feature = "rayon"))]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pairwise_forces: Vec<Vec<((usize, usize), Vector3<f32>)>> = potentials
            .pairs
            .iter()
            .map(|descriptor| {
                descriptor
                    .indices
                    .iter()
                    .map(|(i, j)| {
                        let pos_i = system.positions[*i];
                        let pos_j = system.positions[*j];
                        let r = system.cell().distance(&pos_i, &pos_j);
                        let indices = (*i, *j);
                        let mut force = Vector3::zeros();
                        if descriptor.meta.cutoff > r {
                            let dir = system.cell().direction(&pos_i, &pos_j);
                            force = descriptor.potential.force(r) * dir;
                        }
                        (indices, force)
                    })
                    .collect()
            })
            .collect();

        let mut forces: Vec<Vector3<f32>> = vec![Vector3::zeros(); system.size()];
        pairwise_forces.iter().for_each(|pair_group| {
            pair_group.iter().for_each(|((i, j), force)| {
                forces[*i] += force;
                forces[*j] -= force;
            })
        });
        forces
    }

    #[cfg(feature = "rayon")]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let pairwise_forces: Vec<Vec<((usize, usize), Vector3<f32>)>> = potentials
            .pairs
            .iter()
            .map(|descriptor| {
                descriptor
                    .indices
                    .par_iter()
                    .map(|(i, j)| {
                        let pos_i = system.positions[*i];
                        let pos_j = system.positions[*j];
                        let r = system.cell().distance(&pos_i, &pos_j);
                        let indices = (*i, *j);
                        let mut force = Vector3::zeros();
                        if descriptor.meta.cutoff > r {
                            let dir = system.cell().direction(&pos_i, &pos_j);
                            force = descriptor.potential.force(r) * dir;
                        }
                        (indices, force)
                    })
                    .collect()
            })
            .collect();

        let mut forces: Vec<Vector3<f32>> = vec![Vector3::zeros(); system.size()];
        pairwise_forces.iter().for_each(|pair_group| {
            pair_group.iter().for_each(|((i, j), force)| {
                forces[*i] += force;
                forces[*j] -= force;
            })
        });
        forces
    }
}

/// Potential energy of the whole system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Res = f32;

    #[cfg(not(feature = "rayon"))]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pairs
            .iter()
            .map(|descriptor| {
                let _sum: f32 = descriptor
                    .indices
                    .iter()
                    .map(|(i, j)| {
                        let pos_i = system.positions[*i];
                        let pos_j = system.positions[*j];
                        let r = system.cell().distance(&pos_i, &pos_j);
                        let mut energy = 0 as f32;
                        if descriptor.meta.cutoff > r {
                            energy = descriptor.potential.energy(r)
                        }
                        energy
                    })
                    .sum();
                _sum
            })
            .sum()
    }

    #[cfg(feature = "rayon")]
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        potentials
            .pairs
            .iter()
            .map(|descriptor| {
                let _sum: f32 = descriptor
                    .indices
                    .par_iter()
                    .map(|(i, j)| {
                        let pos_i = system.positions[*i];
                        let pos_j = system.positions[*j];
                        let r = system.cell().distance(&pos_i, &pos_j);
                        let mut energy = 0 as f32;
                        if descriptor.meta.cutoff > r {
                            energy = descriptor.potential.energy(r)
                        }
                        energy
                    })
                    .sum();
                _sum
            })
            .sum()
    }
}

/// Kinetic energy of the whole system
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KineticEnergy;

impl IntrinsicProperty for KineticEnergy {
    type Res = f32;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let kinetic_energy: f32 = system
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
    type Res = f32;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let kinetic = KineticEnergy.calculate(system, potentials);
        let potential = PotentialEnergy.calculate(system, potentials);
        kinetic + potential
    }
}

/// Instantaneous temperature of the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Temperature;

impl IntrinsicProperty for Temperature {
    type Res = f32;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let kinetic = KineticEnergy.calculate_intrinsic(system);
        // NOTE: Calculating DOF this way is a potentially nasty bug if future
        // support is added for degrees of freedom beyond just 3D particles.
        let dof = (system.size() * 3) as f32;
        2.0 * kinetic / (dof * BOLTZMANN)
    }
}
