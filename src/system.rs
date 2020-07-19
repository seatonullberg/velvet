use crate::ensemble::Ensemble;
use crate::simulation::SimulationState;

pub struct System {
    timestep: f32,
    n_timesteps: u32,
    ensemble: Ensemble,
    state: SimulationState,
}
