use nalgebra::{Dynamic, MatrixMN, U3};

#[derive(Clone, Debug)]
struct SimulationState {
    positions: MatrixMN<f32, Dynamic, U3>,
    velocities: MatrixMN<f32, Dynamic, U3>,
    forces: MatrixMN<f32, Dynamic, U3>,
}
