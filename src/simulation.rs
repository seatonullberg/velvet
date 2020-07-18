use nalgebra::{Dynamic, MatrixMN, Vector3, U3};

#[derive(Clone, Debug)]
struct SimulationCell {
    boundaries: Vector3<f32>,
    positions: MatrixMN<f32, Dynamic, U3>,
    velocities: MatrixMN<f32, Dynamic, U3>,
    forces: MatrixMN<f32, Dynamic, U3>,
}
