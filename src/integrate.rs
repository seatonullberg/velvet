use nalgebra::{Dynamic, MatrixMN, U3};

use crate::system::System;

pub trait Integrator {
    fn integrate(
        &self,
        system: &System,
    ) -> (MatrixMN<f32, Dynamic, U3>, MatrixMN<f32, Dynamic, U3>);
}

pub struct VelocityVerlet;
pub struct Beeman;
