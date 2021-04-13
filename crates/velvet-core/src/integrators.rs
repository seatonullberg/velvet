//! Algorithms to integrate classical equations of motion.

use nalgebra::Vector3;

use crate::internal::Float;
use crate::potentials::collections::Potentials;
use crate::properties::forces::Forces;
use crate::properties::Property;
use crate::system::System;

/// A numerical integration algorithm.
pub trait Integrator: Send + Sync {
    /// Prepare the integrator to run.
    fn setup(&mut self, _: &System, _: &Potentials) {}
    /// Integrates one time step.
    fn integrate(&mut self, system: &mut System, potentials: &Potentials);
}

/// Velocity Verlet integration algorithm.
#[derive(Clone, Debug)]
pub struct VelocityVerlet {
    timestep: Float,
    accelerations: Vec<Vector3<Float>>,
}

impl VelocityVerlet {
    /// Returns a new velocity verlet algorithm.
    ///
    /// # Arguments
    ///
    /// * `timestep` - Timestep duration
    pub fn new(timestep: Float) -> VelocityVerlet {
        VelocityVerlet {
            timestep,
            accelerations: Vec::new(),
        }
    }
}

impl Integrator for VelocityVerlet {
    fn setup(&mut self, system: &System, _: &Potentials) {
        self.accelerations = vec![Vector3::zeros(); system.size];
    }

    fn integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let dt = self.timestep;

        system
            .positions
            .iter_mut()
            .zip(system.velocities.iter())
            .zip(self.accelerations.iter())
            .for_each(|((pos, vel), acc)| {
                *pos += (vel * dt) + (0.5 * acc * dt.powi(2));
            });

        let forces = Forces.calculate(system, potentials);
        let new_accelerations: Vec<Vector3<Float>> = forces
            .iter()
            .zip(system.specie_indices.iter())
            .map(|(f, id)| f / system.species[*id].mass())
            .collect();

        system
            .velocities
            .iter_mut()
            .zip(self.accelerations.iter())
            .zip(new_accelerations.iter())
            .for_each(|((vel, acc), new_acc)| {
                *vel += 0.5 * dt * (acc + new_acc);
            });

        self.accelerations = new_accelerations;
    }
}
