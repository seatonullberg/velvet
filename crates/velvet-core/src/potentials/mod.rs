//! Classical interatomic potentials.

pub mod collections;
pub mod coulomb;
pub mod functions;
pub mod pair;

/// Base trait for all potentials.
pub trait Potential: Send + Sync {}
