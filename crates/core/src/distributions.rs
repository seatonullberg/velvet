//! Statistical distribution algorithms.

use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};

use crate::constants::BOLTZMANN;
use crate::properties::{IntrinsicProperty, Temperature};
use crate::system::System;

/// Shared behavior for algorithms that can initialize a velocity distribution.
pub trait VelocityDistribution {
    /// Applies the distribution to a system.
    fn apply(&self, system: &mut System);
}

/// Maxwell Boltzmann style velocity distribution.
pub struct Boltzmann {
    target: f32,
    distr: Normal<f32>,
}

impl Boltzmann {
    /// Returns a new Boltzmann velocity distribution.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    pub fn new(target: f32) -> Boltzmann {
        let distr = Normal::new(0.0, f32::sqrt(BOLTZMANN * target)).unwrap();
        Boltzmann { target, distr }
    }
}

impl VelocityDistribution for Boltzmann {
    fn apply(&self, system: &mut System) {
        for i in 0..system.size() {
            let inv_mass = 1.0 / &system.elements[i].mass();
            let x = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
            let y = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
            let z = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
            system.velocities[i] = Vector3::new(x, y, z);
        }
        scale(system, self.target);
    }
}

/// Scale all velocities in system to the target value.
fn scale(system: &mut System, target: f32) {
    let temperature = Temperature.calculate_intrinsic(system);
    let factor = f32::sqrt(target / temperature);
    system.velocities = system
        .velocities
        .iter_mut()
        .map(|&mut x| x * factor)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{Boltzmann, VelocityDistribution};
    use crate::properties::{IntrinsicProperty, Temperature};
    use crate::utils::load_test_system;
    use approx::*;

    #[test]
    fn boltzmann() {
        // define the system
        let mut sys = load_test_system("argon");

        // define the velocity distribution
        let target = 1000 as f32;
        let initial_temperature = Boltzmann::new(target);

        // apply the initial temperature
        initial_temperature.apply(&mut sys);

        // check the result
        let res = Temperature.calculate_intrinsic(&sys);
        assert_relative_eq!(res, target, epsilon = 1e-3);
    }
}
