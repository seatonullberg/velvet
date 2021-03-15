//! Useful physical constants expressed in the internal unit system.

#[cfg(not(feature = "f64"))]
pub use std::f32::consts::{FRAC_2_SQRT_PI, PI};
#[cfg(feature = "f64")]
pub use std::f64::consts::{FRAC_2_SQRT_PI, PI};

#[cfg(feature = "f64")]
pub const BOLTZMANN: f64 = 0.001985875;
/// Boltzmann constant expressed in kcal/(mol*K).
#[cfg(not(feature = "f64"))]
pub const BOLTZMANN: f32 = 0.001985875;

#[cfg(feature = "f64")]
pub const FOUR_PI_EPSILON_0: f64 = 7.197_59;
/// Coulomb energy constant.
#[cfg(not(feature = "f64"))]
pub const FOUR_PI_EPSILON_0: f32 = 7.197_59;
