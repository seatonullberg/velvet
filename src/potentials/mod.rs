//! Interatomic potential functions.

pub(crate) mod internal;
pub mod models;
pub mod pair;

use std::collections::HashMap;

pub use models::*;
pub use pair::PairPotential;

/// Shared behavior for all potentials.
pub trait Potential<'a>: TryFrom<&'a HashMap<&'a str, f64>> {
    /// Returns the names of parameters used by this potential.
    fn parameter_names() -> Vec<String>;

    /// Returns a mapping of the names and values of the parameters used to evaluate this potential.
    fn parameters(&self) -> HashMap<String, f64>;
}
