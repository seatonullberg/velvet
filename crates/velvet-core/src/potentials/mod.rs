//! Classical interatomic potentials.

pub mod coulomb;
pub mod pair;

use crate::internal::Float;
use crate::potentials::coulomb::{CoulombPotential, CoulombPotentialMeta};
use crate::potentials::pair::{PairPotential, PairPotentialMeta};
use crate::system::species::Species;
use crate::system::System;

/// Base trait for all potentials.
pub trait Potential: Send + Sync {}

pub struct Potentials {
    pub(crate) coulomb_meta: Option<CoulombPotentialMeta>,
    pub(crate) pair_metas: Vec<PairPotentialMeta>,
    pub(crate) update_frequency: usize,
}

impl Potentials {
    pub fn setup(&mut self, system: &System) {
        // setup coulomb potential if it exists
        match &mut self.coulomb_meta {
            Some(meta) => meta.setup(system),
            None => {}
        }
        // setup each pair potential
        self.pair_metas
            .iter_mut()
            .for_each(|meta| meta.setup(system))
    }

    pub fn update(&mut self, system: &System, iteration: usize) {
        // only update if the update frequency is reached
        if iteration % self.update_frequency != 0 {
            return;
        }
        // update coulomb potential if it exists
        match &mut self.coulomb_meta {
            Some(meta) => meta.update(system),
            None => {}
        }
        // update each pair potential
        self.pair_metas
            .iter_mut()
            .for_each(|meta| meta.update(system))
    }
}

pub struct PotentialsBuilder {
    coulomb_meta: Option<CoulombPotentialMeta>,
    pair_metas: Vec<PairPotentialMeta>,
    update_frequency: usize,
}

impl PotentialsBuilder {
    pub fn new() -> PotentialsBuilder {
        PotentialsBuilder {
            coulomb_meta: None,
            pair_metas: Vec::new(),
            update_frequency: 1,
        }
    }

    pub fn coulomb<T>(mut self, potential: T, cutoff: Float, thickness: Float) -> PotentialsBuilder
    where
        T: CoulombPotential + 'static,
    {
        self.coulomb_meta = Some(CoulombPotentialMeta::new(potential, cutoff, thickness));
        self
    }

    pub fn pair<T>(
        mut self,
        potential: T,
        species: (Species, Species),
        cutoff: Float,
        thickness: Float,
    ) -> PotentialsBuilder
    where
        T: PairPotential + 'static,
    {
        self.pair_metas.push(PairPotentialMeta::new(
            potential,
            species,
            cutoff,
            thickness,
        ));
        self
    }

    pub fn update_frequency(mut self, freq: usize) -> PotentialsBuilder {
        self.update_frequency = freq;
        self
    }

    pub fn build(self) -> Potentials {
        Potentials {
            coulomb_meta: self.coulomb_meta,
            pair_metas: self.pair_metas,
            update_frequency: self.update_frequency,
        }
    }
}

/// [Buckingham](https://lammps.sandia.gov/doc/pair_buck.html#description) potential.
#[derive(Clone, Copy, Debug)]
pub struct Buckingham {
    /// Energy units.
    pub a: Float,
    /// Distance units.
    pub rho: Float,
    /// Energy units.
    pub c: Float,
}

impl Buckingham {
    /// Returns a new [`Buckingham`] potential.
    pub fn new(a: Float, rho: Float, c: Float) -> Buckingham {
        Buckingham { a, rho, c }
    }
}

impl Potential for Buckingham {}

/// [Damped Shifted Force](https://lammps.sandia.gov/doc/pair_coul.html#description) potential.
#[derive(Clone, Copy, Debug)]
pub struct DampedShiftedForce {
    /// Damping parameter.
    pub alpha: Float,
    /// Cutoff radius
    pub cutoff: Float,
}

impl DampedShiftedForce {
    /// Returns a new [`DampedShiftedForce`] potential.
    pub fn new(alpha: Float, cutoff: Float) -> DampedShiftedForce {
        DampedShiftedForce {alpha, cutoff}
    }
}

impl Potential for DampedShiftedForce {}



/// [Harmonic](https://lammps.sandia.gov/doc/bond_harmonic.html#description) oscillator potential.
#[derive(Clone, Copy, Debug)]
pub struct Harmonic {
    /// Spring constant.
    pub k: Float,
    /// Equilibrium displacement distance.
    pub x0: Float,
}

impl Harmonic {
    /// Returns a new [`Harmonic`] potential.
    pub fn new(k: Float, x0: Float) -> Harmonic {
        Harmonic { k, x0 }
    }
}

impl Potential for Harmonic {}

/// [Lennard-Jones](https://lammps.sandia.gov/doc/pair_lj.html#description) 12/6 potential.
#[derive(Clone, Copy, Debug)]
pub struct LennardJones {
    /// Depth of the potential well.
    pub epsilon: Float,
    /// Distance at which the pair potential energy is zero.
    pub sigma: Float,
}

impl LennardJones {
    /// Returns a new [`Lennard-Jones`] potential.
    pub fn new(epsilon: Float, sigma: Float) -> LennardJones {
        LennardJones { epsilon, sigma }
    }
}

impl Potential for LennardJones {}

/// [Mie](https://lammps.sandia.gov/doc/pair_mie.html#description) potential.
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    /// Depth of the potential well.
    pub epsilon: Float,
    /// Distance at which the pair potential energy is zero.
    pub sigma: Float,
    /// Exponent on the attractive term.
    pub gamma_a: Float,
    /// Exponent on the repulsize term.
    pub gamma_r: Float,
}

impl Mie {
    /// Returns a new [`Mie`] potential.
    pub fn new(epsilon: Float, sigma: Float, gamma_a: Float, gamma_r: Float) -> Mie {
        Mie {
            epsilon,
            sigma,
            gamma_a,
            gamma_r,
        }
    }
}

impl Potential for Mie {}

/// [Morse](https://lammps.sandia.gov/doc/pair_morse.html#description) potential.
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
    /// Returns a new [`Morse`] potential.
    pub fn new(a: Float, d_e: Float, r_e: Float) -> Morse {
        Morse { a, d_e, r_e }
    }
}

impl Potential for Morse {}

/// Standard [Coulombic](https://lammps.sandia.gov/doc/pair_coul.html#description) potential.
#[derive(Clone, Copy, Debug)]
pub struct StandardCoulombic {
    /// Dielectric constant (unitless).
    pub dielectric: Float,
}

impl StandardCoulombic {
    /// Returns a new [`StandardCoulombic`] potential.
    pub fn new(dielectric: Float) -> StandardCoulombic {
        StandardCoulombic { dielectric }
    }
}

impl Potential for StandardCoulombic {}
