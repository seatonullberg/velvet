//! Algorithms to control the temperature of a simulation.

use crate::potential::Potentials;
use crate::property::{Property, Temperature};
use crate::system::System;

/// An algorithm used to control simulation temperature.
pub trait Thermostat {
    /// Prepare the thermostat to run.
    fn setup(&mut self, _: &System, _: &Potentials) {}
    /// Fires before the integration step.
    fn pre_integrate(&mut self, _: &mut System, _: &Potentials) {}
    /// Fires after the integration step.
    fn post_integrate(&mut self, _: &mut System, _: &Potentials) {}
}

/// Berendsen weak coupling thermostat.
pub struct Berendsen {
    /// Target temperature.
    target: f32,
    /// Timestep of the thermostat expressed as a multiplicative factor 
    /// of the integrator's timestep.
    tau: f32,
}

impl Berendsen {
    /// Returns a new `BerendsenThermostat`.
    pub fn new(target: f32, tau: f32) -> Berendsen {
        Berendsen {target, tau}
    }
}

impl Thermostat for Berendsen {
    fn post_integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let temperature = Temperature.calculate(system, potentials);
        let factor = f32::sqrt(1.0 + (self.target / temperature - 1.0) / self.tau);
        system.velocities = system.velocities.iter_mut().map(|&mut x| x * factor).collect();
    }
}

#[cfg(test)]
mod tests {
    use approx::*;
    use crate::integrate::{Integrator, VelocityVerlet};
    use crate::thermostat::{Berendsen, Thermostat};
    use crate::property::{Property, Temperature};
    use crate::{load_test_potentials, load_test_system};

    #[test]
    fn berendsen() {
        // define the system
        let mut sys = load_test_system!("argon");

        // define the potentials
        let pots = load_test_potentials!("argon");

        // define the integrator
        let mut vv = VelocityVerlet::new(1.0);
        vv.setup(&sys, &pots);
        
        // define the thermostat
        let mut berendsen = Berendsen::new(1000.0, 2.0);
        
        for _ in 0..5000 {
            vv.integrate(&mut sys, &pots);
            berendsen.post_integrate(&mut sys, &pots);
        }

        // check that the simulation was stable
        assert_relative_eq!(Temperature.calculate(&sys, &pots), 1000.0, epsilon = 1e-5);

    }
}
