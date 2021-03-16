#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

#[cfg(feature = "rayon")]
pub type Rc<T> = std::sync::Arc<T>;

#[cfg(not(feature = "rayon"))]
pub type Rc<T> = std::rc::Rc<T>;

pub mod consts {
    #[cfg(not(feature = "f64"))]
    pub use std::f32::consts::{FRAC_2_SQRT_PI, PI};

    #[cfg(feature = "f64")]
    pub use std::f64::consts::{FRAC_2_SQRT_PI, PI};

    #[cfg(feature = "f64")]
    pub const BOLTZMANN: f64 = 0.001985875;

    #[cfg(not(feature = "f64"))]
    pub const BOLTZMANN: f32 = 0.001985875;

    #[cfg(feature = "f64")]
    pub const FOUR_PI_EPSILON_0: f64 = 7.197_59;

    #[cfg(not(feature = "f64"))]
    pub const FOUR_PI_EPSILON_0: f32 = 7.197_59;
}
