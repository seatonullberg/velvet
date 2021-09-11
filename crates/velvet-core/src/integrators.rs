//! Algorithms which integrate the classical equations of motion.

use nalgebra::Vector3;

use crate::internal::Float;
use crate::potentials::Potentials;
use crate::properties::forces::Forces;
use crate::properties::Property;
use crate::system::System;

/// Shared behavior for algorithms which integrate the classical equations of motion.
pub trait Integrator: Send + Sync {
    /// Prepares the integrator to run.
    fn setup(&mut self, _: &System, _: &Potentials) {}
    /// Integrates one timestep.
    fn integrate(&mut self, system: &mut System, potentials: &Potentials);
}

/// Velocity Verlet integration algorithm.
///
/// # References
///
/// [1] Swope, William C., et al. "A computer simulation method for the calculation of equilibrium
/// constants for the formation of physical clusters of molecules: Application to small water clusters."
/// The Journal of chemical physics 76.1 (1982): 637-649.
#[derive(Clone, Debug)]
pub struct VelocityVerlet {
    timestep: Float,
}

impl VelocityVerlet {
    /// Returns a new [`VelocityVerlet`] algorithm.
    ///
    /// # Arguments
    ///
    /// * `timestep` - Timestep duration.
    pub fn new(timestep: Float) -> VelocityVerlet {
        VelocityVerlet { timestep }
    }
}

impl Integrator for VelocityVerlet {
    fn integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let dt = self.timestep;

        system
            .positions
            .iter_mut()
            .zip(system.velocities.iter())
            .zip(system.accelerations.iter())
            .for_each(|((pos, vel), acc)| {
                *pos += (vel * dt) + (0.5 * acc * dt.powi(2));
            });

        let forces = Forces.calculate(system, potentials);
        let new_accelerations: Vec<Vector3<Float>> = forces
            .iter()
            .zip(system.species.iter())
            .map(|(f, species)| f / species.mass())
            .collect();

        system
            .velocities
            .iter_mut()
            .zip(system.accelerations.iter())
            .zip(new_accelerations.iter())
            .for_each(|((vel, acc), new_acc)| {
                *vel += 0.5 * dt * (acc + new_acc);
            });

        system.accelerations = new_accelerations;
    }
}
