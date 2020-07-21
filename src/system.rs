use nalgebra::Vector3;

use crate::ensemble::Ensemble;

#[derive(Clone, Debug)]
pub struct Atom {
    symbol: String,
    charge: f32,
    mass: f32,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
}

#[derive(Clone, Debug)]
pub struct System {
    timestep: f32,
    n_timesteps: u32,
    n_jobs: u32,
    ensemble: Ensemble,
    atoms: Vec<Atom>,
}
