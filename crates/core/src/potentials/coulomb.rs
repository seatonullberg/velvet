/// Electrostatic interaction potentials.

#[cfg(feature = "f64")]
use libm::erfc;
#[cfg(not(feature = "f64"))]
use libm::erfcf as erfc;

use serde::{Deserialize, Serialize};

use crate::constants::{FOUR_PI_EPSILON_0, FRAC_2_SQRT_PI, PI};
use crate::internal::Float;
use crate::potentials::Potential;
use crate::system::System;

#[typetag::serde(tag = "type")]
pub trait CoulombPotential: Potential {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float;
    fn energy_self(&self, qi: Float) -> Float;
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoulombMeta {
    pub cutoff: Float,
    pub indices: Vec<(usize, usize)>,
}

impl CoulombMeta {
    pub fn new(cutoff: Float, system: &System) -> CoulombMeta {
        let mut indices = Vec::with_capacity(system.size * system.size);
        for i in 0..system.size {
            for j in (i + 1)..system.size {
                indices.push((i, j));
            }
        }
        indices.shrink_to_fit();

        CoulombMeta { cutoff, indices }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Wolf {
    alpha: Float,
    energy_constant: Float,
    force_constant: Float,
}

impl Wolf {
    pub fn new(cutoff: Float) -> Wolf {
        let alpha = PI / cutoff;
        let alpha_cut = alpha * cutoff;
        let alpha_cut_2 = alpha_cut * alpha_cut;
        let energy_constant = erfc(alpha_cut) / cutoff;
        let force_constant = erfc(alpha_cut) / (cutoff * cutoff)
            + FRAC_2_SQRT_PI * alpha * -alpha_cut_2.exp() / cutoff;
        Wolf {
            alpha,
            energy_constant,
            force_constant,
        }
    }
}

#[typetag::serde]
impl Potential for Wolf {}

#[typetag::serde]
impl CoulombPotential for Wolf {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        qi * qj * (erfc(self.alpha * r) / r - self.energy_constant) / FOUR_PI_EPSILON_0
    }

    fn energy_self(&self, qi: Float) -> Float {
        qi * qi * 0.5 * (self.energy_constant + self.alpha * FRAC_2_SQRT_PI) / FOUR_PI_EPSILON_0
    }

    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        let r2 = r * r;
        let alpha_r = self.alpha * r;
        let exp_alpha_r = Float::exp(-alpha_r * alpha_r);
        let factor = erfc(alpha_r) / r2 + self.alpha * FRAC_2_SQRT_PI * exp_alpha_r / r;
        qi * qj * (factor - self.force_constant) / (r * FOUR_PI_EPSILON_0)
    }
}

#[cfg(test)]
mod tests {
    use super::{CoulombPotential, Wolf};
    use approx::*;

    #[test]
    fn wolf() {
        let wolf = Wolf::new(8.0);

        let energy_self_na = wolf.energy_self(1.0);
        let energy_self_cl = wolf.energy_self(-1.0);
        let energy_wolf = wolf.energy(1.0, -1.0, 1.5);
        let energy_target = -0.09263977;
        assert_relative_eq!(
            energy_wolf - energy_self_na - energy_self_cl,
            energy_target,
            epsilon = 1e-2
        );

        let force_wolf_a = wolf.force(-1.0, 1.0, 1.5);
        let force_wolf_b = wolf.force(1.0, -1.0, 1.5);
        assert_relative_eq!(force_wolf_a - force_wolf_b, 0.0);
    }
}
