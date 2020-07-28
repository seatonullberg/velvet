use nalgebra::Vector3;

pub trait PairPotential {
    fn energy(&self, r: f32) -> f32;
    fn force(&self, a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32>;
}

/// Lennard-Jones potential
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

    fn force(&self, a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
        let r = (a - b).norm();
        let term_a = (48.0 * self.sigma.powi(12) * self.epsilon) / r.powi(13);
        let term_b = (24.0 * self.sigma.powi(6) * self.epsilon) / r.powi(7);
        let force = term_a - term_b;
        (force * (a - b)) / r
    }
}

/// Mie potential
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

    fn force(&self, a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
        let r = (a - b).norm();
        let c = (self.gamma_r / (self.gamma_r - self.gamma_a))
            * (self.gamma_r / self.gamma_a).powf(self.gamma_a / (self.gamma_r - self.gamma_a));
        let term_a = (c * self.gamma_r * self.epsilon * (self.sigma / r).powf(self.gamma_r)) / r;
        let term_b = (c * self.gamma_a * self.epsilon * (self.sigma / r).powf(self.gamma_a)) / r;
        let force = term_a - term_b;
        (force * (a - b)) / r
    }
}

/// Morse potential
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

    fn force(&self, a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
        let r = (a - b).norm();
        let term_a = f32::exp(-2.0 * self.a * (r - self.r_e));
        let term_b = f32::exp(-self.a * (r - self.r_e));
        let force = 2.0 * self.a * self.d_e * (term_a - term_b);
        (force * (a - b)) / r
    }
}
