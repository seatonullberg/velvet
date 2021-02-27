/// Electrostatic interaction potentials.

#[cfg(feature = "f64")]
use libm::erfc;

#[cfg(not(feature = "f64"))]
use libm::erfcf as erfc;

use serde::{Deserialize, Serialize};

use crate::constants::PI;
use crate::internal::Float;
use crate::potentials::Potential;

#[typetag::serde(tag = "type")]
pub trait CoulombPotential: Potential {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float;
    fn energy_self(&self, qi: Float) -> Float;
    fn force(&self, qi: Float, qj: Float, r: Float) -> Float;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Wolf {
    alpha: Float,
    cutoff: Float,
}

impl Wolf {
    pub fn new(alpha: Float, cutoff: Float) -> Wolf {
        Wolf {
            alpha,
            cutoff,
        }
    }
}

#[typetag::serde]
impl Potential for Wolf {}

#[typetag::serde]
impl CoulombPotential for Wolf {
    fn energy(&self, qi: Float, qj: Float, r: Float) -> Float {
        let term_a = erfc(self.alpha * r) / r;
        let term_b = erfc(self.alpha * self.cutoff) / self.cutoff;
        qi * qj *  (term_a - term_b)
    }

    fn energy_self(&self, qi: Float) -> Float {
        let term_a = 0.5 * erfc(self.alpha * self.cutoff) / self.cutoff;
        let term_b = self.alpha / Float::sqrt(PI);
        -(qi * qi) * (term_a + term_b)
    }

    fn force(&self, qi: Float, qj: Float, r: Float) -> Float {
        let r2 = r * r;
        let term_a = -erfc(self.alpha * r) / r2;
        let term_b = 2.0 * self.alpha * Float::exp(-(self.alpha * self.alpha * r2)) / (Float::sqrt(PI) * r);
        qi * qj * (term_a - term_b)
    }
}

#[cfg(test)]
mod tests {
    use super::{CoulombPotential, Wolf};
    use approx::*;

    #[test]
    fn wolf() {
        let wolf = Wolf::new(0.25, 8.0);

        let energy_wolf = wolf.energy(1.0, -1.0, 1.5);
        let energy_target = -0.396671;
        assert_relative_eq!(energy_wolf, energy_target, epsilon=1e-5);

        let force_wolf = wolf.force(1.0, -1.0, 1.5);
        let force_target = 0.428229;
        assert_relative_eq!(force_wolf, force_target, epsilon=1e-5);

        let energy_self_wolf = wolf.energy_self(1.0);
        let energy_self_target = -0.141340;
        assert_relative_eq!(energy_self_wolf, energy_self_target, epsilon=1e-5);
    }
}
