pub trait PairPotential {
    fn energy(&self, r: f32) -> f32;
}

/// Lennard-Jones potential
///
/// $$
/// V(r) = 4\epsilon \Bigg[\Big(\frac \sigma r\Big)^{12} - \Big(\frac \sigma r\Big)^6\Bigg]
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct LennardJones {
    epsilon: f32,
    sigma: f32,
}

impl PairPotential for LennardJones {
    fn energy(&self, r: f32) -> f32 {
        let term = (self.sigma / r).powi(6);
        4.0 * self.epsilon * (term * term - term)
    }
}

/// Morse potential
///
/// $$
/// V(r) = D_e\Big(e^{-2a(r - r_e)} - 2e^{-a(r - r_e)}\Big)
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Morse {
    a: f32,
    de: f32,
    re: f32,
}

impl PairPotential for Morse {
    fn energy(&self, r: f32) -> f32 {
        let term_a = f32::exp(-2.0 * self.a * (r - self.re));
        let term_b = 2.0 * f32::exp(-self.a * (r - self.re));
        self.de * (term_a - term_b)
    }
}
