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

pub mod neighbors;
pub mod outputs;
pub mod potentials;
pub mod propagator;
pub mod properties;
pub mod simulation;

/// User facing exports.
pub mod prelude {
    // TODO
}
