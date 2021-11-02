//! Algorithms which control the temperature of a system.

use velvet_core::potentials::Potentials;
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::Property;
use velvet_internals::float::Float;
use velvet_system::System;

/// Shared behavior for algorithms which control the temperature of a system.
pub trait Thermostat {
    /// Prepares the thermostat to run.
    fn setup(&mut self, system: &System);
    /// Fires before the integration step.
    fn pre_integrate(&mut self, system: &mut System, potentials: &Potentials);
    /// Fires after the integration step.
    fn post_integrate(&mut self, system: &mut System, potentials: &Potentials);
}

/// Berendsen weak coupling thermostat.
///
/// # References
///
/// [[1](https://www.tandfonline.com/doi/abs/10.1080/08927029408021981)]
/// Lemak, A. S., and N. K. Balabaev.
/// "On the Berendsen thermostat."
/// Molecular Simulation 13.3 (1994): 177-187.
///
/// [[2](https://www2.mpip-mainz.mpg.de/~andrienk/journal_club/thermostats.pdf)]
/// Rühle, Victor.
/// "Berendsen and nose-hoover thermostats."
/// Am. J. Phys (2007).
#[derive(Clone, Copy, Debug)]
pub struct Berendsen {
    target: Float,
    tau: Float,
}

impl Berendsen {
    /// Returns a new [Berendsen] thermostat.
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
    fn setup(&mut self, _: &System) {}

    fn pre_integrate(&mut self, _: &mut System, _: &Potentials) {}

    fn post_integrate(&mut self, system: &mut System, potentials: &Potentials) {
        let temperature = Temperature.calculate(system, potentials);
        let factor = Float::sqrt(1.0 + (self.target / temperature - 1.0) / self.tau);
        system.velocities = system.velocities.iter().map(|&v| v * factor).collect();
    }
}

/// Nose-Hoover style thermostat.
///
/// # References
///
/// [[1](https://aip.scitation.org/doi/pdf/10.1063/1.449071?casa_token=7EbU0aNT6n8AAAAA:IwXjvKHFkOrcMt5TFIyu7A_bx-W7wmTtd5sD_xIpjooDqPUYUMdmQsvglQm-pMoyJ7HfCqXUn0V0)]
/// Evans, Denis J., and Brad Lee Holian.
/// "The nose–hoover thermostat."
/// The Journal of chemical physics 83.8 (1985): 4069-4074.
///
/// [[2](https://www2.mpip-mainz.mpg.de/~andrienk/journal_club/thermostats.pdf)]
/// Rühle, Victor.
/// "Berendsen and nose-hoover thermostats."
/// Am. J. Phys (2007).
#[derive(Clone, Copy, Debug)]
pub struct NoseHoover {
    target: Float,
    freq: Float,
    timestep: Float,
    psi: Float,
    factor: Float,
    temperature: Float,
}

impl NoseHoover {
    /// Returns a new [NoseHoover] thermostat.
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
    fn setup(&mut self, _: &System) {}

    fn pre_integrate(&mut self, system: &mut System, potentials: &Potentials) {
        self.temperature = Temperature.calculate(system, potentials);
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        let dt = self.timestep;
        self.psi += psidot * (dt / 2.0);
        self.factor = Float::exp(-self.psi * (dt / 2.0));
        system.velocities = system.velocities.iter().map(|&v| v * self.factor).collect();
    }

    fn post_integrate(&mut self, system: &mut System, potentials: &Potentials) {
        self.temperature = Temperature.calculate(system, potentials);
        let psidot = self.freq.powi(2) * ((self.temperature / self.target) - 1.0);
        let dt = self.timestep;
        self.psi += psidot * (dt / 2.0);
    }
}
