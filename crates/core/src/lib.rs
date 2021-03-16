//! Core feature library for the Velvet simulation engine.
//!
//! Internal unit system follows LAMMPS [real](https://lammps.sandia.gov/doc/units.html) style.
//!
//! * `mass` - grams/mole
//! * `distance` - angstrom
//! * `time` - femtosecond
//! * `energy` - Kcal/mole
//! * `force` - Kcal/mole-angstrom
//! * `temperature` - Kelvin
#![warn(missing_docs)]
#![warn(clippy::all)]

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate strum_macros;

pub mod config;
pub mod integrators;
mod internal;
pub mod neighbors;
pub mod outputs;
pub mod potentials;
pub mod propagators;
pub mod properties;
pub mod simulation;
pub mod system;
pub mod thermostats;
pub mod velocity_distributions;

/// User facing exports.
pub mod prelude {
    pub use super::config::{Configuration, ConfigurationBuilder};
    pub use super::integrators::{Integrator, VelocityVerlet};
    pub use super::outputs::*;
    pub use super::potentials::collections::{Potentials, PotentialsBuilder};
    pub use super::potentials::functions::*;
    pub use super::potentials::coulomb::CoulombPotential;
    pub use super::potentials::pair::PairPotential;
    pub use super::potentials::Potential;
    pub use super::propagators::{MolecularDynamics, Propagator};
    pub use super::properties::energy::*;
    pub use super::properties::forces::*;
    pub use super::properties::temperature::*;
    pub use super::properties::{IntrinsicProperty, Property};
    pub use super::simulation::Simulation;
    pub use super::system::cell::Cell;
    pub use super::system::elements::Element;
    pub use super::system::species::Specie;
    pub use super::system::System;
    pub use super::thermostats::{Berendsen, NoseHoover, NullThermostat, Thermostat};
    pub use super::velocity_distributions::{Boltzmann, VelocityDistribution};
}
