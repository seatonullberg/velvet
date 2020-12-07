//! Algorithms to control the temperature of a simulation.

use crate::system::System;

/// An algorithm used to control simulation temperature.
pub trait Thermostat {
    /// Prepare the thermostat to run.
    fn setup(&mut self, _: &System) {}
    /// Modify the temperature of the system by adjusting its velocities.
    fn apply(&mut self, system: &mut System);
}
