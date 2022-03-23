//! Velvet is a classical atomistic simulation engine with a focus on user-friendliness and extensibility.
//!
//! This project is largely a learning exercise, but as development continues I hope to accomplish the following goals:
//!
//! * Extensibility via user-defined plugin modules
//! * Optimized single CPU performace with multithreading and SIMD support
//! * Implement a wide variety of interatomic potentials
//! * Molecular dynamics, Monte Carlo, and energy minimization routines
//! * Visualization tools to analyze simulation results
//! * Support importing and exporting data in popular external formats

/// `use velvet::prelude::*;` to import common components.
pub mod prelude {
    pub use velvet_core::prelude::*;
}

pub mod chemfiles {
    pub use velvet_chemfiles::*;
}

pub mod core {
    //! Core feature library for the Velvet simulation engine.
    pub use velvet_core::*;
}

pub mod md {
    pub use velvet_md::*;
}

pub mod system {
    pub use velvet_system::*;
}
