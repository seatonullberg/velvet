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

/// Mie potential
///
/// $$
/// V(r) = \epsilon\Big[\Big(\frac \sigma r\Big)^{\gamma_r} - \Big(\frac \sigma r\Big)^{\gamma_a}\Big]
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Mie {
    epsilon: f32,
    sigma: f32,
    gamma_a: f32,
    gamma_r: f32,
}

/// Morse potential
///
/// $$
/// V(r) = D_e\Big(e^{-2a(r - r_e)} - 2e^{-a(r - r_e)}\Big)
/// $$
#[derive(Clone, Copy, Debug, Default)]
pub struct Morse {
    a: f32,
    d_e: f32,
    r_e: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Pair {
    LennardJones(LennardJones),
    Mie(Mie),
    Morse(Morse),
}

impl Pair {
    fn energy(&self, r: f32) -> f32 {
        match *self {
            Pair::LennardJones(lj) => {
                let term = (lj.sigma / r).powi(6);
                4.0 * lj.epsilon * (term * term - term)
            }
            Pair::Mie(mie) => {
                let term_a = (mie.sigma / r).powf(mie.gamma_r);
                let term_b = (mie.sigma / r).powf(mie.gamma_a);
                mie.epsilon * (term_a - term_b)
            }
            Pair::Morse(morse) => {
                let term_a = f32::exp(-2.0 * morse.a * (r - morse.r_e));
                let term_b = 2.0 * f32::exp(-morse.a * (r - morse.r_e));
                morse.d_e * (term_a - term_b)
            }
        }
    }
}
