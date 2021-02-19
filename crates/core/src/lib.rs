//! Core feature library for the Velvet simulation engine.
//!
//! Internal unit system follows LAMMPS `real` style.
//!
//! * `Mass` - grams/mole
//! * `Distance` - angstrom
//! * `Time` - femtosecond
//! * `Energy` - Kcal/mole
//! * `Force` - Kcal/mole-angstrom
//! * `Temperature` - Kelvin

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate strum_macros;

#[warn(missing_docs)]
mod internal;
pub mod config;
pub mod constants;
pub mod integrators;
pub mod outputs;
pub mod potentials;
pub mod propagators;
pub mod properties;
pub mod simulation;
pub mod system;
pub mod thermostats;
pub mod velocity_distributions;

pub mod prelude {
    pub use super::config::*;
    pub use super::integrators::*;
    pub use super::outputs::*;
    pub use super::potentials::coulomb::*;
    pub use super::potentials::pair::*;
    pub use super::potentials::*;
    pub use super::propagators::*;
    pub use super::properties::energy::*;
    pub use super::properties::forces::*;
    pub use super::properties::temperature::*;
    pub use super::properties::*;
    pub use super::simulation::*;
    pub use super::system::cell::*;
    pub use super::system::elements::*;
    pub use super::system::*;
    pub use super::thermostats::*;
    pub use super::velocity_distributions::*;
}
