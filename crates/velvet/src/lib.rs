//! Velvet is an open-source classical atomistic simulation engine written in Rust, with a focus on user-friendliness and extensibility.
//!
//! This project is largely a learning exercise, but as development continues I hope to accomplish the following goals:
//!
//! * Extensibility via user-defined plugin modules
//! * Optimized single CPU performace with multithreading and SIMD support
//! * Implement a wide variety of interatomic potentials
//! * Molecular Dynamics, Monte Carlo, and Minimization routines
//! * Visualization tools to analyze simulation results
//! * Support importing and exporting data in popular external formats

/// `use velvet::prelude::*;` to import common components.
pub mod prelude {
    pub use velvet_core::prelude::*;
    pub use velvet_external_data::prelude::*;
}

pub mod core {
    //! Core feature library for the Velvet simulation engine.
    pub use velvet_core::*;
}

pub mod external_data {
    //! Utilities to import and export data in external formats.
    pub use velvet_external_data::*;
}
