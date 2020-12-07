//! Physical properties of the simulated system.

use nalgebra::Vector3;

use crate::consts::BOLTZMANN;
use crate::potential::{Potentials, Restriction};
use crate::system::System;

/// Calculates a system-wide property.
pub trait Property {
    /// The property's return type.
    type Output;
    /// Returns a physical property of the system.
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Output;
}

/// Force acting on each atom in the system.
#[derive(Clone, Copy, Debug)]
pub struct Forces;

impl Property for Forces {
    type Output = Vec<Vector3<f32>>;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Output {
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
#[derive(Clone, Copy, Debug)]
pub struct PotentialEnergy;

impl Property for PotentialEnergy {
    type Output = f32;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Output {
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
#[derive(Clone, Copy, Debug)]
pub struct KineticEnergy;

impl Property for KineticEnergy {
    type Output = f32;

    fn calculate(&self, system: &System, _: &Potentials) -> Self::Output {
        let sys_size = system.size();
        let mut kinetic_energy = 0.0 as f32;

        for i in 0..sys_size {
            kinetic_energy += 0.5 * system.masses[i] * system.velocities[i].norm_squared();
        }
        kinetic_energy
    }
}

/// Total energy of the system.
#[derive(Clone, Copy, Debug)]
pub struct TotalEnergy;

impl Property for TotalEnergy {
    type Output = f32;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Output {
        let kinetic = KineticEnergy.calculate(system, potentials);
        let potential = PotentialEnergy.calculate(system, potentials);
        kinetic + potential
    }
}

/// Instantaneous temperature of the system.
#[derive(Clone, Copy, Debug)]
pub struct Temperature;

impl Property for Temperature {
    type Output = f32;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Output {
        let kinetic = KineticEnergy.calculate(system, potentials);
        // NOTE: Calculating DOF this way is a potentially nasty bug if future
        // support is added for degrees of freedom beyond just 3D particles.
        let dof = (system.size() * 3) as f32;
        2.0 * kinetic / (dof * BOLTZMANN)
    }
}

#[cfg(test)]
mod tests {
    use crate::property::{Forces, KineticEnergy, PotentialEnergy, Property, TotalEnergy};
    use crate::{load_test_potentials, load_test_system};
    use approx::*;

    use super::Temperature;

    #[test]
    fn forces() {
        // define the system
        let sys = load_test_system!("fluorine");

        // define the potentials
        let pots = load_test_potentials!("fluorine");

        // calculate the forces
        let forces = Forces.calculate(&sys, &pots);
        let total_force = forces[0] + forces[1];
        assert_relative_eq!(total_force.norm(), 0.0);

        let target_force = 30.0 as f32;
        assert_relative_eq!(forces[0][0], -target_force, epsilon = 1e-4);
        assert_relative_eq!(forces[0][1], 0.0);
        assert_relative_eq!(forces[0][2], 0.0);

        assert_relative_eq!(forces[1][0], target_force, epsilon = 1e-4);
        assert_relative_eq!(forces[1][1], 0.0);
        assert_relative_eq!(forces[1][2], 0.0);
    }

    #[test]
    fn energy() {
        // define the system
        let sys = load_test_system!("fluorine");

        // define the potentials
        let pots = load_test_potentials!("fluorine");

        // calculate the energies
        let kinetic = KineticEnergy.calculate(&sys, &pots);
        let potential = PotentialEnergy.calculate(&sys, &pots);
        let total = TotalEnergy.calculate(&sys, &pots);

        assert_eq!(kinetic + potential, total);
        assert_relative_eq!(kinetic, 0.0007483);
    }

    #[test]
    fn temperature() {
        // define the system
        let sys = load_test_system!("fluorine");

        // define the potentials
        let pots = load_test_potentials!("fluorine");

        // calculate the temperature
        let temperature = Temperature.calculate(&sys, &pots);
        assert_relative_eq!(temperature, 300.0, epsilon = 1e-2);
    }
}
