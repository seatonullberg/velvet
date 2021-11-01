//! Velvet's core functionality
//!
//! Internal unit system follows LAMMPS [real](https://lammps.sandia.gov/doc/units.html) style.
//!
//! * `mass` - grams/mole
//! * `distance` - angstrom
//! * `time` - femtosecond
//! * `energy` - Kcal/mole
//! * `force` - Kcal/mole-angstrom
//! * `temperature` - Kelvin

#![warn(clippy::all)]

pub mod neighbors;
pub mod potentials;
pub mod propagators;
pub mod properties;

/// User facing exports.
pub mod prelude {
    // TODO
}
