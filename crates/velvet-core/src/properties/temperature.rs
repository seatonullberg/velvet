use crate::potentials::Potentials;
use crate::properties::energy::KineticEnergy;
use crate::properties::Property;
use velvet_internals::consts::BOLTZMANN;
use velvet_internals::float::Float;
use velvet_system::System;

/// Instantaneous temperature of the system.
#[derive(Clone, Copy, Debug)]
pub struct Temperature;

impl Property for Temperature {
    type T = Float;

    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::T {
        let kinetic = KineticEnergy.calculate(system, potentials);
        // NOTE: This value DOF is only valid for atomic resolution simulations.
        let dof = (system.n_atoms * 3) as Float;
        2.0 * kinetic / (dof * BOLTZMANN)
    }
}
