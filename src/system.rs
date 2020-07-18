use crate::ensemble::Ensemble;

#[derive(Clone, Debug)]
struct System {
    timestep: f32,
    n_timesteps: u32,
    ensemble: Ensemble,
    potentials: Vec<String>,  // TODO
    output: String,           // TODO
    simulation_state: String, // TODO
}
