//! Velvet is a classical atomistic simulation engine with a focus on user-friendliness and portability - two features that I believe are not well represented in the current scientific software environment. This project is largely a learning exercise, but as development continues I hope to accomplish the following goals:
//!
//! * Extensibility via user-defined plugin modules
//! * Optimized single CPU performace with multithreading and SIMD support
//! * Implement a wide variety of interatomic potentials
//! * Molecular Dynamics, Monte Carlo, and Minimization routines
//! * Visualization tools to analyze simulation results

pub use velvet_core as core;
pub use velvet_external_data as external_data;

pub mod prelude {
    pub use super::core::prelude::*;
    pub use super::external_data::prelude::*;
}
