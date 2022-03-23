//! Physical properties of the simulated system.

pub mod energy;
pub mod forces;
pub mod temperature;

use crate::potentials::Potentials;
use velvet_system::System;

/// Calculates a system-wide property.
pub trait Property {
    /// The property's return type.
    type T;

    /// Returns a physical property of the system.
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::T;
}
