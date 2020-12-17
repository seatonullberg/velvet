//! Physical properties of the simulated system.

use nalgebra::Vector3;
use serde::{Serialize, Deserialize};

use crate::constants::BOLTZMANN;
use crate::potentials::{Potentials, Restriction};
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

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let sys_size = system.size();
        let mut forces: Vec<Vector3<f32>> = vec![Vector3::new(0.0, 0.0, 0.0); sys_size];

        // iterate over all pairs of atoms
        for i in 0..sys_size {
            // skip duplicate or identical pairs
            for j in (i + 1)..sys_size {
                // calculate distance between the pair
                let pos1 = &system.positions[i];
                let pos2 = &system.positions[j];
                let r = system.cell.distance(pos1, pos2);

                // iterate over the pair potentials
                for (potential, meta) in potentials.pairs() {
                    // check cutoff radius
                    if meta.cutoff < r {
                        continue;
                    }

                    // check element pair
                    let elem1 = &system.elements[i];
                    let elem2 = &system.elements[j];
                    if (*elem1, *elem2) != meta.elements {
                        continue;
                    }

                    // check restricton
                    let ok = match meta.restriction {
                        Restriction::None => true,
                        Restriction::Intermolecular => &system.molecules[i] != &system.molecules[j],
                        Restriction::Intramolecular => &system.molecules[i] == &system.molecules[j],
                    };
                    if ok {
                        let dir = &system.cell.direction(pos1, pos2);
                        let force = potential.force(r) * dir;
                        forces[i] += force;
                        forces[j] -= force;
                    }
                }
            }
        }
        forces
    }
}

/// Potential energy of the whole system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Res = f32;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res {
        let sys_size = system.size();
        let mut potential_energy: f32 = 0.0 as f32;

        // iterate over all pairs of atoms
        for i in 0..sys_size {
            // skip duplicate or identical pairs
            for j in (i + 1)..sys_size {
                // calculate distance between the pair
                let pos1 = &system.positions[i];
                let pos2 = &system.positions[j];
                let r = system.cell.distance(pos1, pos2);

                // iterate over the pair potentials
                for (potential, meta) in potentials.pairs() {
                    // check cutoff radius
                    if meta.cutoff < r {
                        continue;
                    }

                    // check element pair
                    let elem1 = &system.elements[i];
                    let elem2 = &system.elements[j];
                    if (*elem1, *elem2) != meta.elements {
                        continue;
                    }

                    // check restricton
                    let ok = match meta.restriction {
                        Restriction::None => true,
                        Restriction::Intermolecular => &system.molecules[i] != &system.molecules[j],
                        Restriction::Intramolecular => &system.molecules[i] == &system.molecules[j],
                    };
                    if ok {
                        potential_energy += potential.energy(r);
                    }
                }
            }
        }
        potential_energy
    }
}

/// Kinetic energy of the whole system
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KineticEnergy;

impl IntrinsicProperty for KineticEnergy {
    type Res = f32;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let sys_size = system.size();
        let mut kinetic_energy = 0.0 as f32;

        for i in 0..sys_size {
            kinetic_energy += 0.5 * system.elements[i].mass() * system.velocities[i].norm_squared();
        }
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
