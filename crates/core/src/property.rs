use nalgebra::Vector3;

use crate::system::System;

/// Calculates a system-wide property.
pub trait Property {
    fn calculate(&self, system: &System) -> Output;
}

pub enum PropertyEnum {
    Forces(Forces),
    External(Box<dyn Property>),
}

impl Property for PropertyEnum {
    fn calculate(&self, system: &System) -> Output {
        match self {
            PropertyEnum::Forces(f) => f.calculate(system),
            PropertyEnum::External(e) => e.calculate(system),
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
    fn calculate(&self, system: &System) -> Output {
        unimplemented!()
    }
}
