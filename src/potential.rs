pub trait PairPotential {
    fn energy(&self, r: f32) -> f32;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LennardJones {
    epsilon: f32,
    sigma: f32,
}

impl PairPotential for LennardJones {
    fn energy(&self, f: f32) -> f32 {
        let term = (self.sigma / r).powi(6);
        4 * self.epsilon * (term * term - term)
    }
}
