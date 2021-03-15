//! Classical interatomic potentials.

pub mod collections;
pub mod coulomb;
pub mod functions;
pub mod interactions;
pub mod pair;

/// Base trait for all potentials.
#[typetag::serde(tag = "type")]
pub trait Potential: Send + Sync {}
