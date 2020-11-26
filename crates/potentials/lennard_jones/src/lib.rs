use velvet_core::{export_plugin, PluginRegistrar};
use velvet_core::potential::PairPotential;

#[derive(Clone, Copy, Debug, Default)]
pub struct LennardJones {
    /// Depth of the potential energy well.
    epsilon: f32,
    /// Finite distance at which the potential evaluates to zero.
    sigma: f32,
}

impl PairPotential for LennardJones {
    fn energy(&self, r: f32) -> f32 {
        let term = (self.sigma / r).powi(6);
        4.0 * self.epsilon * (term * term - term)
    }

    fn force(&self, r: f32) -> f32 {
        let term_a = (48.0 * self.sigma.powi(12) * self.epsilon) / r.powi(13);
        let term_b = (24.0 * self.sigma.powi(6) * self.epsilon) / r.powi(7);
        term_a - term_b
    }
}

export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_pair_potential("LennardJones", Box::new(LennardJones::default()));
}

#[cfg(test)]
mod tests {
    use velvet_core::potential::PairPotential;
    use crate::LennardJones;

    #[test]
    fn energy() {
        let lj = LennardJones {
            epsilon: 0.8,
            sigma: 2.0,
        };
        assert_eq!(lj.energy(2.0), 0.0);
        assert_eq!(lj.energy(2.5), -0.6189586);
    }

    #[test]
    fn force() {
        let lj = LennardJones {
            epsilon: 0.8,
            sigma: 2.0,
        };
        assert!(lj.force(f32::powf(2.0, 1.0 / 6.0) * 2.0).abs() < 1e-6);
        assert_eq!(lj.force(2.5), -0.9577348);
    }
}