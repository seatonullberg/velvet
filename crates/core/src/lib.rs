//! Core feature library for the Velvet simulation engine.
//!
//! Internal unit system:
//!
//! * `Distance` - Angstrom (A)
//! * `Time` - Femtosecond (fs)
//! * `Mass` - Atomic mass unit (amu)
//! * `Temperature` - Kelvin (K)
//! * `Quantity` - Number of particles
//! * `Angle` - Radians (rad)

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[warn(missing_docs)]
pub mod config;
pub mod constants;
pub mod distributions;
pub mod integrators;
pub mod outputs;
pub mod potentials;
pub mod propagators;
pub mod properties;
pub mod simulation;
pub mod system;
pub mod thermostats;

pub mod prelude {
    pub use super::config::*;
    pub use super::distributions::*;
    pub use super::integrators::*;
    pub use super::outputs::*;
    pub use super::potentials::*;
    pub use super::potentials::pair::*;
    pub use super::propagators::*;
    pub use super::properties::*;
    pub use super::simulation::*;
    pub use super::system::*;
    pub use super::system::cell::*;
    pub use super::system::elements::*;
    pub use super::thermostats::*;
}