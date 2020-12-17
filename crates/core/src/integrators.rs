//! Algorithms to integrate classical equations of motion.

use nalgebra::Vector3;

use crate::potentials::Potentials;
use crate::properties::{Forces, Property};
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
    timestep: f32,
    accelerations: Vec<Vector3<f32>>,
}

impl VelocityVerlet {
    /// Returns a new velocity verlet algorithm.
    ///
    /// # Arguments
    ///
    /// * `timestep` - Timestep duration
    pub fn new(timestep: f32) -> VelocityVerlet {
        VelocityVerlet {
            timestep,
            accelerations: Vec::new(),
        }
    }
}

impl Integrator for VelocityVerlet {
    fn setup(&mut self, system: &System, _: &Potentials) {
        self.accelerations = vec![Vector3::default(); system.size()];
    }

    fn integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let dt = self.timestep;
        let sys_size = system.size();

        // update velocities at t + dt/2 and positions at t + dt
        for i in 0..sys_size {
            system.velocities[i] += 0.5 * dt * self.accelerations[i];
            system.positions[i] += system.velocities[i] * dt;
        }

        // calculate forces
        let forces = Forces.calculate(system, potentials);

        // update accelerations at t + dt
        for i in 0..sys_size {
            self.accelerations[i] = forces[i] / system.elements[i].mass();
        }

        // update velocities at t + dt
        for i in 0..sys_size {
            system.velocities[i] += 0.5 * dt * self.accelerations[i];
        }
    }
}
