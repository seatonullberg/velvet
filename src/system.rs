use crate::ensemble::Ensemble;
use crate::potential::Potential;
use crate::simulation::SimulationState;

pub struct System {
    timestep: f32,
    n_timesteps: u32,
    n_jobs: u32,
    ensemble: Ensemble,
    state: SimulationState,
}

fn propagate(system: &System, potentials: Vec<Potential>) {}
