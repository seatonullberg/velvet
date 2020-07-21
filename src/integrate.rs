use nalgebra::{Dynamic, MatrixMN, U3};

use crate::system::System;

pub trait Integrator {
    fn integrate(
        &self,
        system: &mut System,
    ) -> (MatrixMN<f32, Dynamic, U3>, MatrixMN<f32, Dynamic, U3>);
}

pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn integrate(
        &self,
        system: &mut System,
    ) -> (MatrixMN<f32, Dynamic, U3>, MatrixMN<f32, Dynamic, U3>) {
        unimplemented!();
    }
}

pub struct Beeman;

impl Integrator for Beeman {
    fn integrate(
        &self,
        system: &mut System,
    ) -> (MatrixMN<f32, Dynamic, U3>, MatrixMN<f32, Dynamic, U3>) {
        unimplemented!();
    }
}
