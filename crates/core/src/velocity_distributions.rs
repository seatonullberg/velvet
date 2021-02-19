//! Statistical distribution algorithms.

use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};

use crate::constants::BOLTZMANN;
use crate::properties::temperature::Temperature;
use crate::properties::IntrinsicProperty;
use crate::system::System;

/// Shared behavior for algorithms that can initialize a velocity distribution.
pub trait VelocityDistribution: Send + Sync {
    /// Applies the distribution to a system.
    fn apply(&self, system: &mut System);
}

/// Maxwell Boltzmann style velocity distribution.
#[derive(Clone, Copy, Debug)]
pub struct Boltzmann {
    target: f64,
    distr: Normal<f64>,
}

impl Boltzmann {
    /// Returns a new Boltzmann velocity distribution.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    pub fn new(target: f64) -> Boltzmann {
        let distr = Normal::new(0.0, f64::sqrt(BOLTZMANN * target)).unwrap();
        Boltzmann { target, distr }
    }
}

impl VelocityDistribution for Boltzmann {
    fn apply(&self, system: &mut System) {
        // !!! this block is more efficient without `par_iter`
        system.velocities = system
            .elements
            .iter()
            .map(|elem| {
                let inv_mass = 1.0 / elem.mass();
                let x = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                let y = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                let z = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                Vector3::new(x, y, z)
            })
            .collect::<Vec<Vector3<f64>>>();
        scale(system, self.target);
    }
}

/// Scale all velocities in system to the target value.
fn scale(system: &mut System, target: f64) {
    let temperature = Temperature.calculate_intrinsic(system);
    let factor = f64::sqrt(target / temperature);
    // !!! this block is more efficient without `par_iter`
    system.velocities = system.velocities.iter().map(|&x| x * factor).collect();
}
