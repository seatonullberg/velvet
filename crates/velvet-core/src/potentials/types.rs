//! Types of interatomic potential functions.

use velvet_internals::float::Float;

/// Buckingham potential.
/// This implementation is based on the LAMMPS pair style [`buck`](https://lammps.sandia.gov/doc/pair_buck.html#description).
///
/// # References
///
/// [[1](https://royalsocietypublishing.org/doi/pdf/10.1098/rspa.1938.0173)]
/// Buckingham, Richard A. "The classical equation of state of gaseous helium, neon and argon."
/// Proceedings of the Royal Society of London. Series A. Mathematical and Physical Sciences 168.933 (1938): 264-283.
#[derive(Clone, Copy, Debug)]
pub struct Buckingham {
    /// Repulsive parameter.
    pub a: Float,
    /// Pair dependent length parameter.
    pub rho: Float,
    /// Attractive parameter.
    pub c: Float,
}

impl Buckingham {
    /// Returns a new [Buckingham] potential.
    pub fn new(a: Float, rho: Float, c: Float) -> Buckingham {
        Buckingham { a, rho, c }
    }
}

/// Harmonic oscillator potential.
/// This implementation is based on the LAMMPS bond style [`harmonic`](https://lammps.sandia.gov/doc/bond_harmonic.html#description).
#[derive(Clone, Copy, Debug)]
pub struct Harmonic {
    /// Spring constant.
    pub k: Float,
    /// Equilibrium separation distance.
    pub x0: Float,
}

impl Harmonic {
    /// Returns a new [Harmonic] potential.
    pub fn new(k: Float, x0: Float) -> Harmonic {
        Harmonic { k, x0 }
    }
}

/// Lennard-Jones 12/6 potential.
/// This implementation is based on the LAMMPS pair style [`lj/cut`](https://lammps.sandia.gov/doc/pair_lj.html#description).
///
/// # References
///
/// [[1](https://royalsocietypublishing.org/doi/pdf/10.1098/rspa.1924.0081)]
/// Jones, John Edward. "On the determination of molecular fields.—I. From the variation of the viscosity of a gas with temperature."
/// Proceedings of the Royal Society of London. Series A, Containing Papers of a Mathematical and Physical Character 106.738 (1924): 441-462.
///
/// [[2](https://royalsocietypublishing.org/doi/pdf/10.1098/rspa.1924.0082)]
/// Jones, John Edward. "On the determination of molecular fields.—II. From the equation of state of a gas."
/// Proceedings of the Royal Society of London. Series A, Containing Papers of a Mathematical and Physical Character 106.738 (1924): 463-477.
#[derive(Clone, Copy, Debug)]
pub struct LennardJones {
    /// Depth of the potential well.
    pub epsilon: Float,
    /// Distance at which the pair potential energy is zero.
    pub sigma: Float,
}

impl LennardJones {
    /// Returns a new [LennardJones] potential.
    pub fn new(epsilon: Float, sigma: Float) -> LennardJones {
        LennardJones { epsilon, sigma }
    }
}

/// Mie potential.
/// This implementation is based on the LAMMPS pair style [`mie/cut`](https://lammps.sandia.gov/doc/pair_mie.html#description).
///
/// # References
///
/// [[1](https://ia800708.us.archive.org/view_archive.php?archive=/22/items/crossref-pre-1909-scholarly-works/10.1002%252Fandp.19023130716.zip&file=10.1002%252Fandp.19033160802.pdf)]
/// Mie, Gustav. "Zur kinetischen Theorie der einatomigen Körper." Annalen der Physik 316.8 (1903): 657-697.
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    /// Depth of the potential well.
    pub epsilon: Float,
    /// Distance at which the pair potential energy is zero.
    pub sigma: Float,
    /// Exponent on the attractive term.
    pub gamma_a: Float,
    /// Exponent on the repulsive term.
    pub gamma_r: Float,
}

impl Mie {
    /// Returns a new [Mie] potential.
    pub fn new(epsilon: Float, sigma: Float, gamma_a: Float, gamma_r: Float) -> Mie {
        Mie {
            epsilon,
            sigma,
            gamma_a,
            gamma_r,
        }
    }
}

/// Morse potential.
/// This implementation is based on the LAMMPS pair style [`morse`](https://lammps.sandia.gov/doc/pair_morse.html#description).
///
/// # References
///
/// [[1](https://journals.aps.org/pr/pdf/10.1103/PhysRev.34.57?casa_token=TAvD_Hm5X38AAAAA%3AybhPJAioRTlZMafe2FlIvNA8HYyWN9sIX5RsT9i7kI47e2NcL0ywpe7vTX6utz-38LkFrG8RemRP1VU)]
/// Morse, Philip M. "Diatomic molecules according to the wave mechanics. II. Vibrational levels." Physical review 34.1 (1929): 57.
#[derive(Clone, Copy, Debug)]
pub struct Morse {
    /// Width of the potential well.
    pub a: Float,
    /// Depth of the potential well.
    pub d_e: Float,
    /// Equilibrium bond distance.
    pub r_e: Float,
}

impl Morse {
    /// Returns a new [Morse] potential.
    pub fn new(a: Float, d_e: Float, r_e: Float) -> Morse {
        Morse { a, d_e, r_e }
    }
}
