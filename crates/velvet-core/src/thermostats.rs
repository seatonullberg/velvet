//! Algorithms which control the temperature of a system.

use nalgebra::Vector3;

use crate::internal::Float;
use crate::properties::temperature::Temperature;
use crate::properties::IntrinsicProperty;
use crate::system::System;

/// Shared behavior for algorithms which control the temperature of a system.
pub trait Thermostat: Send + Sync {
    /// Prepares the thermostat to run.
    fn setup(&mut self, _: &System) {}
    /// Fires before the integration step.
    fn pre_integrate(&mut self, _: &mut System) {}
    /// Fires after the integration step.
    fn post_integrate(&mut self, _: &mut System) {}
}

/// Mock thermostat algorithm which applies no temperature controls.
#[derive(Clone, Debug)]
pub struct NullThermostat;

impl Thermostat for NullThermostat {}

/// Berendsen weak coupling thermostat.
///
/// # References
///
/// [1] Lemak, A. S., and N. K. Balabaev. "On the Berendsen thermostat." Molecular Simulation 13.3 (1994): 177-187.
///
/// [2] Rühle, Victor. "Berendsen and nose-hoover thermostats." Am. J. Phys (2007).
#[derive(Clone, Debug)]
pub struct Berendsen {
    target: Float,
    tau: Float,
}

impl Berendsen {
    /// Returns a new Berendsen style thermostat.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature.
    /// * `tau` - Timestep of the thermostat expressed as a multiple of the integrator's timestep.
    pub fn new(target: Float, tau: Float) -> Berendsen {
        Berendsen { target, tau }
    }
}

impl Thermostat for Berendsen {
    fn post_integrate(&mut self, system: &mut System) {
        let temperature = Temperature.calculate_intrinsic(system);
        let factor = Float::sqrt(1.0 + (self.target / temperature - 1.0) / self.tau);
        system.velocities = system
            .velocities
            .iter()
            .map(|&v| v * factor)
            .collect::<Vec<Vector3<Float>>>();
    }
}

/// Nose-Hoover style thermostat.
///
/// # References
///
/// [1] Evans, Denis J., and Brad Lee Holian. "The nose–hoover thermostat." The Journal of chemical physics 83.8 (1985): 4069-4074.
///
/// [2] Rühle, Victor. "Berendsen and nose-hoover thermostats." Am. J. Phys (2007).
#[derive(Clone, Debug)]
pub struct NoseHoover {
    target: Float,
    freq: Float,
    timestep: Float,
    psi: Float,
    factor: Float,
    temperature: Float,
}

impl NoseHoover {
    /// Returns a new Nose-Hoover style thermostat.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature.
    /// * `freq` - Damping frequency.
    /// * `timestep` - Timestep of the integrator.
    pub fn new(target: Float, freq: Float, timestep: Float) -> NoseHoover {
        NoseHoover {
            target,
            freq,
            timestep,
            psi: 0 as Float,
            factor: 0 as Float,
            temperature: 0 as Float,
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
        self.factor = Float::exp(-self.psi * (dt / 2.0));
        system.velocities = system
            .velocities
            .iter()
            .map(|&v| v * self.factor)
            .collect::<Vec<Vector3<Float>>>();
    }

    fn post_integrate(&mut self, system: &mut System) {
        let dt = self.timestep;
        self.temperature = Temperature.calculate_intrinsic(system);
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        self.psi += psidot * (dt / 2.0);
    }
}
