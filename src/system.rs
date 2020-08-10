use nalgebra::{Matrix3, Vector3};

use crate::ensemble::Ensemble;

#[derive(Clone, Debug)]
pub struct Atom {
    pub symbol: String,
    pub charge: f32,
    pub mass: f32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

#[derive(Clone, Debug)]
pub struct System {
    pub atoms: Vec<Atom>,
    pub basis: Matrix3<f32>,
    pub ensemble: Ensemble,
    pub n_threads: u32,
    pub n_timesteps: u32,
    pub periodicity: Vector3<bool>,
    pub timestep: f32,
}
