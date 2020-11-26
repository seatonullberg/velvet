use crate::procedure::Procedure;
use crate::simcell::SimulationCell;

pub struct Configuration {
    pub n_threads: i32,
    pub output_frequency: i32,
    pub procedure: Procedure,
    pub simulation_cell: SimulationCell,
}