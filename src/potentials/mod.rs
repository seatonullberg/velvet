pub mod pair;

use std::collections::HashMap;

/// Shared behavior and marker trait for all potentials.
pub trait Potential<'a>: From<HashMap<&'a str, f64>> {
    /// Returns a mapping of the names and values of the parameters used to evaluate this potential.
    fn parameters(&self) -> HashMap<String, f64>;
}
