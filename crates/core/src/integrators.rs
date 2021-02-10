//! Algorithms to integrate classical equations of motion.

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::potentials::Potentials;
use crate::properties::{Forces, Property};
use crate::system::System;

/// A numerical integration algorithm.
#[typetag::serde(tag = "type")]
pub trait Integrator: Send + Sync {
    /// Prepare the integrator to run.
    fn setup(&mut self, _: &System, _: &Potentials) {}
    /// Integrates one time step.
    fn integrate(&mut self, system: &mut System, potentials: &Potentials);
}

/// Velocity Verlet integration algorithm.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[typetag::serde]
impl Integrator for VelocityVerlet {
    fn setup(&mut self, system: &System, _: &Potentials) {
        self.accelerations = vec![Vector3::default(); system.size()];
    }

    fn integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let dt = self.timestep;

        // update velocities at t + dt/2
        system.set_velocities(system
            .iter_velocities()
            .zip(self.accelerations.iter())
            .map(|(&v, &acc)| v + (0.5 * dt * acc))
            .collect::<Vec<Vector3<f32>>>()
        );

        // update positions at t + dt
        system.set_positions(
            system
                .iter_positions()
                .zip(system.iter_velocities())
                .map(|(&p, &v)| p + (v * dt))
                .collect(),
        );

        // calculate forces
        let forces = Forces.calculate(system, potentials);

        // update accelerations at t + dt
        self.accelerations = forces
            .iter()
            .zip(system.iter_elements())
            .map(|(&f, &elem)| f / elem.mass())
            .collect();

        // update velocities at t + dt
        system.set_velocities(system
            .iter_velocities()
            .zip(self.accelerations.iter())
            .map(|(&v, &acc)| v + (0.5 * dt * acc))
            .collect::<Vec<Vector3<f32>>>()
        );
    }
}
