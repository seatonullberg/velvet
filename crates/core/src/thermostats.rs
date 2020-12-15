//! Algorithms to control the temperature of a simulation.

use crate::properties::{IntrinsicProperty, Temperature};
use crate::system::System;

/// An algorithm used to control simulation temperature.
pub trait Thermostat {
    /// Prepare the thermostat to run.
    fn setup(&mut self, _: &System) {}
    /// Fires before the integration step.
    fn pre_integrate(&mut self, _: &mut System) {}
    /// Fires after the integration step.
    fn post_integrate(&mut self, _: &mut System) {}
}

/// Berendsen weak coupling thermostat.
pub struct Berendsen {
    target: f32,
    tau: f32,
}

impl Berendsen {
    /// Returns a new Berendsen style thermostat.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    /// * `tau` - Timestep of the thermostat expressed as a multiple of the integrator's timestep
    pub fn new(target: f32, tau: f32) -> Berendsen {
        Berendsen { target, tau }
    }
}

impl Thermostat for Berendsen {
    fn post_integrate(&mut self, system: &mut System) {
        let temperature = Temperature.calculate_intrinsic(system);
        let factor = f32::sqrt(1.0 + (self.target / temperature - 1.0) / self.tau);
        system.velocities = system
            .velocities
            .iter_mut()
            .map(|&mut x| x * factor)
            .collect();
    }
}

pub struct NoseHoover {
    target: f32,
    freq: f32,
    timestep: f32,
    psi: f32,
    factor: f32,
    temperature: f32,
}

impl NoseHoover {
    /// Returns a new Nose-Hoover style thermostat.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    /// * `freq` - Damping frequency
    /// * `timestep` - Simulation timestep
    pub fn new(target: f32, freq: f32, timestep: f32) -> NoseHoover {
        NoseHoover { target, freq, timestep, psi: 0 as f32, factor: 0 as f32, temperature: 0 as f32 }
    }
}

impl Thermostat for NoseHoover {

    fn setup(&mut self, system: &System) {
        self.temperature = Temperature.calculate_intrinsic(system);
    }

    fn pre_integrate(&mut self, system: &mut System) {
        //println!("pre temperature: {:?}", self.temperature);
        let dt = self.timestep;
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        //println!("pre psidot: {:?}", psidot);
        self.psi += psidot * (dt / 2.0);
        //println!("pre psi: {:?}", self.psi);
        self.factor = f32::exp(-self.psi * (dt / 2.0));
        //println!("pre factor: {:?}", self.factor);
        system.velocities = system.velocities.iter().map(|&x| x * self.factor).collect();
        //println!("pre velocities: {:?}\n", system.velocities);
    }

    fn post_integrate(&mut self, system: &mut System) {
        let dt = self.timestep;
        //system.velocities = system.velocities.iter().map(|&x| x * self.factor).collect();
        //println!("post velocities: {:?}", system.velocities);
        self.temperature = Temperature.calculate_intrinsic(system);
        //println!("post temperature: {:?}", self.temperature);
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        //println!("post psidot: {:?}", psidot);
        self.psi += psidot * (dt / 2.0);
        //println!("post psi: {:?}\n", self.psi);
    }
}

#[cfg(test)]
mod tests {
    use super::{Berendsen, Thermostat, NoseHoover};
    use crate::{distributions::{Boltzmann, VelocityDistribution}, integrators::{Integrator, VelocityVerlet}};
    use crate::properties::{Property, Temperature};
    use crate::utils::{load_test_potentials, load_test_system};
    use approx::*;

    #[test]
    fn berendsen() {
        // define the system
        let mut sys = load_test_system("argon");

        // define the potentials
        let pots = load_test_potentials("argon");

        // define the integrator
        let mut vv = VelocityVerlet::new(1.0);
        vv.setup(&sys, &pots);

        // define the thermostat
        let target = 1000 as f32;
        let mut berendsen = Berendsen::new(target, 2.0);

        // run the integration with a thermostat
        for _ in 0..5000 {
            vv.integrate(&mut sys, &pots);
            berendsen.post_integrate(&mut sys);
        }

        // check that the simulation was stable
        assert_relative_eq!(Temperature.calculate(&sys, &pots), target, epsilon = 1e-5);
    }

    #[test]
    fn nose_hoover() {
        // define the system
        let mut sys = load_test_system("argon");

        let boltz = Boltzmann::new(100 as f32);
        boltz.apply(&mut sys);

        // define the potentials
        let pots = load_test_potentials("argon");

        // define the integrator
        let mut vv = VelocityVerlet::new(1.0);
        vv.setup(&sys, &pots);

        // define the thermostat
        let target = 300 as f32;
        let mut nose = NoseHoover::new(target, 1.0 + 1e-2, 1.0);
        nose.setup(&sys);

        // run the integration with a thermostat
        for _ in 0..5000 {
            nose.pre_integrate(&mut sys);
            vv.integrate(&mut sys, &pots);
            nose.post_integrate(&mut sys);
        }

        // check that the simulation was stable
        assert_relative_eq!(Temperature.calculate(&sys, &pots), target, epsilon = 100.0);
    }
}
