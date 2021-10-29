pub mod consts {
    #[cfg(not(feature = "f64"))]
    pub use std::f32::consts::{FRAC_2_SQRT_PI, PI};
    #[cfg(feature = "f64")]
    pub use std::f64::consts::{FRAC_2_SQRT_PI, PI};

    pub const BOLTZMANN: crate::float::Float = 0.001985875;
    pub const COULOMB: crate::float::Float = 332.0636;
}

pub mod float {
    #[cfg(feature = "f64")]
    pub type Float = f64;

    #[cfg(not(feature = "f64"))]
    pub type Float = f32;
}
