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

#[macro_use]
extern crate strum_macros;

pub mod config;
pub mod integrators;
mod internal;
pub mod outputs;
pub mod potentials;
pub mod propagators;
pub mod properties;
pub mod selection;
pub mod simulation;
pub mod system;
pub mod thermostats;
pub mod velocity_distributions;

/// User facing exports.
pub mod prelude {
    pub use super::config::*;
    pub use super::integrators::*;
    #[cfg(feature = "hdf5-output")]
    pub use super::outputs::hdf5::*;
    pub use super::outputs::raw::*;
    pub use super::outputs::*;
    pub use super::potentials::coulomb::*;
    pub use super::potentials::pair::*;
    pub use super::potentials::types::*;
    pub use super::potentials::*;
    pub use super::propagators::*;
    pub use super::properties::energy::*;
    pub use super::properties::forces::*;
    pub use super::properties::temperature::*;
    pub use super::properties::*;
    pub use super::selection::*;
    pub use super::simulation::*;
    pub use super::system::cell::*;
    pub use super::system::elements::*;
    pub use super::system::particle::*;
    pub use super::system::*;
    pub use super::thermostats::*;
    pub use super::velocity_distributions::*;
}
