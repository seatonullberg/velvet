use crate::system::System;

pub trait Integrator {
    fn integrate(&self, system: &mut System);
}

pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn integrate(&self, system: &mut System) {
        unimplemented!()
    }
}

pub struct Beeman;

impl Integrator for Beeman {
    fn integrate(&self, system: &mut System) {
        unimplemented!()
    }
}
