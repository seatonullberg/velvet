//! Algorithms which initialize the temperature of a sytem from a velocity distribution.

use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};
use velvet_core::potentials::Potentials;
use velvet_core::properties::temperature::Temperature;
use velvet_core::properties::Property;
use velvet_internals::consts::BOLTZMANN;
use velvet_internals::float::Float;
use velvet_system::System;

/// Shared behavior for algorithms which define a distribution of atomic velocities to initialize temperature.
pub trait VelocityDistribution {
    /// Applies the distribution to a system.
    fn apply(&self, system: &mut System, potentials: &Potentials);
}

/// Maxwell-Boltzmann style velocity distribution.
///
/// # References
///
/// [[1](https://www.tandfonline.com/doi/pdf/10.1080/002068970500044749?casa_token=gDaJF0_n3F0AAAAA:rqzlAq3Q5emnpUWBsH7EZMpzfFyjxweW0_VpmDzbm1qipXXr49WAo-zJHIjN7H5roNlBfHxh9IYTHg)]
/// Rowlinson*, J. S.
/// "The Maxwell–Boltzmann distribution."
/// Molecular Physics 103.21-23 (2005): 2821-2828.
#[derive(Clone, Copy, Debug)]
pub struct Boltzmann {
    target: Float,
    distr: Normal<Float>,
}

impl Boltzmann {
    /// Returns a new [Boltzmann] velocity distribution.
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
    fn apply(&self, system: &mut System, potentials: &Potentials) {
        system.velocities = system
            .species
            .iter()
            .map(|species| {
                let inv_mass: Float = 1.0 / species.mass();
                let x: Float = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                let y: Float = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                let z: Float = inv_mass.sqrt() * self.distr.sample(&mut rand::thread_rng());
                Vector3::new(x, y, z)
            })
            .collect();
        scale(system, potentials, self.target);
    }
}

/// Scale all velocities to the target value.
fn scale(system: &mut System, potentials: &Potentials, target: Float) {
    let temperature = Temperature.calculate(system, potentials);
    let factor = Float::sqrt(target / temperature);
    system.velocities = system.velocities.iter().map(|&x| x * factor).collect();
}
