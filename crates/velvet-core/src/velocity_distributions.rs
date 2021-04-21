//! Algorithms which initialize the temperature of a sytem from a velocity distribution.

use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};

use crate::internal::consts::BOLTZMANN;
use crate::internal::Float;
use crate::properties::temperature::Temperature;
use crate::properties::IntrinsicProperty;
use crate::system::System;

/// Shared behavior for algorithms which initialize the temperature of a system from a velocity distribution.
pub trait VelocityDistribution: Send + Sync {
    /// Applies the distribution to a system.
    fn apply(&self, system: &mut System);
}

/// Maxwell-Boltzmann style velocity distribution.
///
/// # References
///
/// [1] Hernandez, Hugo. "Standard Maxwell-Boltzmann distribution: definition and properties." ForsChem Research Reports 2 (2017): 2017-2.
#[derive(Clone, Copy, Debug)]
pub struct Boltzmann {
    target: Float,
    distr: Normal<Float>,
}

impl Boltzmann {
    /// Returns a new [`Boltzmann`] velocity distribution.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature.
    pub fn new(target: Float) -> Boltzmann {
        let distr = Normal::new(0.0, Float::sqrt(BOLTZMANN * target)).unwrap();
        Boltzmann { target, distr }
    }
}

impl VelocityDistribution for Boltzmann {
    fn apply(&self, system: &mut System) {
        system.velocities = system
            .particle_type_map
            .iter()
            .map(|idx| {
                let pt = system.particle_types[*idx];
                let inv_mass = 1.0 / pt.mass();
                let x = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                let y = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                let z = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                Vector3::new(x, y, z)
            })
            .collect::<Vec<Vector3<Float>>>();
        scale(system, self.target);
    }
}

/// Scale all velocities in system to the target value.
fn scale(system: &mut System, target: Float) {
    let temperature = Temperature.calculate_intrinsic(system);
    let factor = Float::sqrt(target / temperature);
    system.velocities = system.velocities.iter().map(|&x| x * factor).collect();
}
