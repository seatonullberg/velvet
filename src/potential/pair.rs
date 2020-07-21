pub trait PairPotential {
    fn energy(&self, r: f32) -> f32;
    // fn force(&self, r: f32) -> f32;
}

/// Lennard-Jones potential
///
/// $$
/// V(r) = 4\epsilon \Bigg[\Big(\frac \sigma r\Big)^{12} - \Big(\frac \sigma r\Big)^6\Bigg]
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct LennardJones {
    /// Depth of the potential well.
    epsilon: f32,
    /// Finite distance at which the potential evaluates to zero.
    sigma: f32,
}

impl PairPotential for LennardJones {
    fn energy(&self, r: f32) -> f32 {
        let term = (self.sigma / r).powi(6);
        4.0 * self.epsilon * (term * term - term)
    }
}

/// Mie potential
///
/// $$
/// V(r) = C\epsilon\Big[\Big(\frac \sigma r\Big)^{\gamma_r} - \Big(\frac \sigma r\Big)^{\gamma_a}\Big]
/// $$
///
/// Where C is a function:
///
/// $$
/// C = \Bigg(\frac {\gamma_r} {\gamma_r - \gamma_a}\Bigg)\Bigg(\frac {\gamma_r} {\gamma_a}\Bigg)^{\big(\frac {\gamma_a} {\gamma_r - \gamma_a}\big)}
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Mie {
    epsilon: f32,
    sigma: f32,
    gamma_a: f32,
    gamma_r: f32,
}

impl PairPotential for Mie {
    fn energy(&self, r: f32) -> f32 {
        let term_a = (self.sigma / r).powf(self.gamma_r);
        let term_b = (self.sigma / r).powf(self.gamma_a);
        let c = (self.gamma_r / (self.gamma_r - self.gamma_a))
            * (self.gamma_r / self.gamma_a).powf(self.gamma_a / (self.gamma_r - self.gamma_a));
        c * self.epsilon * (term_a - term_b)
    }
}

/// Morse potential
///
/// $$
/// V(r) = D_e\Big(e^{-2a(r - r_e)} - 2e^{-a(r - r_e)}\Big)
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Morse {
    /// Potential well width.
    a: f32,
    /// Potential well depth.
    d_e: f32,
    /// Equilibrium bond distance.
    r_e: f32,
}

impl PairPotential for Morse {
    fn energy(&self, r: f32) -> f32 {
        let term_a = f32::exp(-2.0 * self.a * (r - self.r_e));
        let term_b = 2.0 * f32::exp(-self.a * (r - self.r_e));
        self.d_e * (term_a - term_b)
    }
}
