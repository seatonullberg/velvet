//! Algorithms to control the temperature of a simulation.

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::properties::temperature::Temperature;
use crate::properties::IntrinsicProperty;
use crate::system::System;

/// An algorithm used to control simulation temperature.
#[typetag::serde(tag = "type")]
pub trait Thermostat: Send + Sync {
    /// Prepare the thermostat to run.
    fn setup(&mut self, _: &System) {}
    /// Fires before the integration step.
    fn pre_integrate(&mut self, _: &mut System) {}
    /// Fires after the integration step.
    fn post_integrate(&mut self, _: &mut System) {}
}

/// Placeholder thermostat algorithm which applies no temperature controls.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NullThermostat;

#[typetag::serde]
impl Thermostat for NullThermostat {}

/// Berendsen weak coupling thermostat.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Berendsen {
    target: f64,
    tau: f64,
}

impl Berendsen {
    /// Returns a new Berendsen style thermostat.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    /// * `tau` - Timestep of the thermostat expressed as a multiple of the integrator's timestep
    pub fn new(target: f64, tau: f64) -> Berendsen {
        Berendsen { target, tau }
    }
}

#[typetag::serde]
impl Thermostat for Berendsen {
    fn post_integrate(&mut self, system: &mut System) {
        let temperature = Temperature.calculate_intrinsic(system);
        let factor = f64::sqrt(1.0 + (self.target / temperature - 1.0) / self.tau);

        // !!! this block is more efficient without `par_iter`
        system.velocities = system
            .velocities
            .iter()
            .map(|&v| v * factor)
            .collect::<Vec<Vector3<f64>>>();
    }
}

/// Nose-Hoover style thermostat.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoseHoover {
    target: f64,
    freq: f64,
    timestep: f64,
    psi: f64,
    factor: f64,
    temperature: f64,
}

impl NoseHoover {
    /// Returns a new Nose-Hoover style thermostat.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    /// * `freq` - Damping frequency
    /// * `timestep` - Simulation timestep
    pub fn new(target: f64, freq: f64, timestep: f64) -> NoseHoover {
        NoseHoover {
            target,
            freq,
            timestep,
            psi: 0 as f64,
            factor: 0 as f64,
            temperature: 0 as f64,
        }
    }
}

#[typetag::serde]
impl Thermostat for NoseHoover {
    fn setup(&mut self, system: &System) {
        self.temperature = Temperature.calculate_intrinsic(system);
    }

    fn pre_integrate(&mut self, system: &mut System) {
        let dt = self.timestep;
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        self.psi += psidot * (dt / 2.0);
        self.factor = f64::exp(-self.psi * (dt / 2.0));

        // !!! this block is more efficient without `par_iter`
        system.velocities = system
            .velocities
            .iter()
            .map(|&v| v * self.factor)
            .collect::<Vec<Vector3<f64>>>();
    }

    fn post_integrate(&mut self, system: &mut System) {
        let dt = self.timestep;
        self.temperature = Temperature.calculate_intrinsic(system);
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        self.psi += psidot * (dt / 2.0);
    }
}
