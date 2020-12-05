use nalgebra::Vector3;

use crate::potential::Potentials;
use crate::system::System;

/// Calculates a system-wide property.
pub trait Property {
    fn calculate(&self, system: &System, potentials: &Potentials) -> Output;
}

pub enum PropertyEnum {
    Forces(Forces),
    External(Box<dyn Property>),
}

impl Property for PropertyEnum {
    fn calculate(&self, system: &System, potentials: &Potentials) -> Output {
        match self {
            PropertyEnum::Forces(f) => f.calculate(system, potentials),
            PropertyEnum::External(e) => e.calculate(system, potentials),
        }
    }
}

/// Valid output types for system properties.
#[derive(Clone, Debug)]
pub enum Output {
    Scalar(f32),
    Vector(Vec<f32>),
    Matrix(Vec<Vector3<f32>>),
}

#[derive(Clone, Debug)]
pub struct Forces;

impl Property for Forces {
    fn calculate(&self, system: &System, potentials: &Potentials) -> Output {
        unimplemented!()
    }
}
