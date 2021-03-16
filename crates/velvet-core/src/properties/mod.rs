//! Physical properties of the simulated system.

pub mod energy;
pub mod forces;
pub mod temperature;

use crate::potentials::collections::Potentials;
use crate::system::System;

/// Calculates a system-wide property.
pub trait Property {
    /// The property's return type.
    type Res;
    /// Returns a physical property of the system.
    fn calculate(&self, system: &System, potentials: &Potentials) -> Self::Res;
}

/// Calculates a system-wide property without using the applied potentials.
pub trait IntrinsicProperty {
    /// The property's return type.
    type Res;
    /// Returns a physical property of the system without accessing the associated potentials.
    fn calculate_intrinsic(&self, system: &System) -> Self::Res;
}

impl<T: IntrinsicProperty> Property for T {
    type Res = T::Res;

    fn calculate(&self, system: &System, _: &Potentials) -> Self::Res {
        <T as IntrinsicProperty>::calculate_intrinsic(&self, system)
    }
}