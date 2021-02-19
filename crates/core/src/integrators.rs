//! Algorithms to integrate classical equations of motion.

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::potentials::Potentials;
use crate::properties::forces::Forces;
use crate::properties::Property;
use crate::system::System;
use crate::internal::Float;

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

#[typetag::serde]
impl Integrator for VelocityVerlet {
    fn setup(&mut self, system: &System, _: &Potentials) {
        self.accelerations = vec![Vector3::zeros(); system.size()];
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
            .zip(system.elements.iter())
            .map(|(f, elem)| f / elem.mass())
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
