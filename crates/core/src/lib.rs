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
