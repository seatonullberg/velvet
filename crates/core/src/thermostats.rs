//! Algorithms to control the temperature of a simulation.

use crate::properties::{IntrinsicProperty, Temperature};
use crate::system::System;

/// An algorithm used to control simulation temperature.
pub trait Thermostat: Send + Sync {
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
        NoseHoover {
            target,
            freq,
            timestep,
            psi: 0 as f32,
            factor: 0 as f32,
            temperature: 0 as f32,
        }
    }
}

impl Thermostat for NoseHoover {
    fn setup(&mut self, system: &System) {
        self.temperature = Temperature.calculate_intrinsic(system);
    }

    fn pre_integrate(&mut self, system: &mut System) {
        let dt = self.timestep;
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        self.psi += psidot * (dt / 2.0);
        self.factor = f32::exp(-self.psi * (dt / 2.0));
        system.velocities = system.velocities.iter().map(|&x| x * self.factor).collect();
    }

    fn post_integrate(&mut self, system: &mut System) {
        let dt = self.timestep;
        self.temperature = Temperature.calculate_intrinsic(system);
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        self.psi += psidot * (dt / 2.0);
    }
}
