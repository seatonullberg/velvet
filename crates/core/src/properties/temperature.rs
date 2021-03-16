use serde::{Deserialize, Serialize};

use crate::internal::Float;
use crate::internal::consts::BOLTZMANN;
use crate::properties::energy::KineticEnergy;
use crate::properties::IntrinsicProperty;
use crate::system::System;

/// Instantaneous temperature of the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Temperature;

impl IntrinsicProperty for Temperature {
    type Res = Float;

    fn calculate_intrinsic(&self, system: &System) -> <Self as IntrinsicProperty>::Res {
        let kinetic = KineticEnergy.calculate_intrinsic(system);
        // NOTE: Calculating DOF this way is a potentially nasty bug if future
        // support is added for degrees of freedom beyond just 3D particles.
        let dof = (system.size * 3) as Float;
        2.0 * kinetic / (dof * BOLTZMANN)
    }
}
