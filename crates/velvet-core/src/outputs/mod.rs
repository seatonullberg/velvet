//! Properties which can be output as a result from the simulation.

#[cfg(feature = "hdf5-output")]
pub mod hdf5;

use crate::potentials::collections::Potentials;
use crate::properties::energy::{KineticEnergy, PairEnergy, PotentialEnergy, TotalEnergy};
use crate::properties::forces::Forces;
use crate::properties::temperature::Temperature;
use crate::properties::Property;
use crate::system::System;

/// Shared behavior to log a simulation result.
#[typetag::serde(tag = "type")]
pub trait Output {
    /// Logs the output.
    fn output(&self, system: &System, potentials: &Potentials);
}

#[typetag::serde]
impl Output for Forces {
    fn output(&self, system: &System, potentials: &Potentials) {
        let forces = self.calculate(system, potentials);
        info!("Forces: {:#?}", forces);
    }
}

#[typetag::serde]
impl Output for KineticEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Kinetic Energy: {:?}", energy);
    }
}

#[typetag::serde]
impl Output for PotentialEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Potential Energy: {:?}", energy);
    }
}

#[typetag::serde]
impl Output for TotalEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Total Energy: {:?}", energy);
    }
}

#[typetag::serde]
impl Output for PairEnergy {
    fn output(&self, system: &System, potentials: &Potentials) {
        let energy = self.calculate(system, potentials);
        info!("Pair Energy: {:?}", energy);
    }
}

#[typetag::serde]
impl Output for Temperature {
    fn output(&self, system: &System, potentials: &Potentials) {
        let temp = self.calculate(system, potentials);
        info!("Temperature: {:?}", temp);
    }
}
