//! Statistical distribution algorithms.

use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};

use crate::consts::BOLTZMANN;
use crate::internal::Float;
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
    target: Float,
    distr: Normal<Float>,
}

impl Boltzmann {
    /// Returns a new Boltzmann velocity distribution.
    ///
    /// # Arguments
    ///
    /// * `target` - Target temperature (Kelvin)
    pub fn new(target: Float) -> Boltzmann {
        let distr = Normal::new(0.0, Float::sqrt(BOLTZMANN * target)).unwrap();
        Boltzmann { target, distr }
    }
}

impl VelocityDistribution for Boltzmann {
    fn apply(&self, system: &mut System) {
        // !!! this block is more efficient without `par_iter`
        system.velocities = system
            .specie_ids
            .iter()
            .map(|id| {
                let sp = system.species[id];
                let inv_mass = 1.0 / sp.mass();
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
    // !!! this block is more efficient without `par_iter`
    system.velocities = system.velocities.iter().map(|&x| x * factor).collect();
}
