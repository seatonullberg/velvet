//! Algorithms to integrate classical equations of motion.

use nalgebra::Vector3;

use crate::potential::Potentials;
use crate::property::{Forces, Property};
use crate::system::System;

/// A numerical integration algorithm.
pub trait Integrator {
    /// Prepare the integrator to run.
    fn setup(&mut self, _: &System, _: &Potentials) {}
    /// Integrates one time step.
    fn integrate(&mut self, system: &mut System, potentials: &Potentials);
}

/// Velocity Verlet integration algorithm.
///
/// Include equations here.
#[derive(Clone, Debug)]
pub struct VelocityVerlet {
    timestep: f32,
    accelerations: Vec<Vector3<f32>>,
}

impl VelocityVerlet {
    /// Returns a new `VelocityVerlet`.
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
            self.accelerations[i] = forces[i] / system.masses[i];
        }

        // update velocities at t + dt
        for i in 0..sys_size {
            system.velocities[i] += 0.5 * dt * self.accelerations[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::integrate::{Integrator, VelocityVerlet};
    use crate::{load_test_potentials, load_test_system};

    #[test]
    fn velocity_verlet() {
        // define the system
        let mut sys = load_test_system!("argon");

        // define the potentials
        let pots = load_test_potentials!("argon");

        // define the integrator
        let mut vv = VelocityVerlet::new(1.0);
        vv.setup(&sys, &pots);
        for _ in 0..5000 {
            vv.integrate(&mut sys, &pots)
        }

        // check that the simulation was stable
        assert!(sys.velocities[0].norm() < 0.1);
    }
}
