//! Results of an atomistic simulation.

// TODO: Really need to find a way to do blanket implementations.

use crate::potentials::Potentials;
use velvet_system::System;

pub mod energy;
pub mod forces;
pub mod temperature;

pub trait Output {
    fn output(&self, system: &System, potentials: &Potentials, timestep: usize) -> String;
}

pub struct OutputMeta {
    output: Box<dyn Output>,
    writer: Box<dyn std::io::Write>,
    pub interval: usize,
}

impl OutputMeta {
    pub fn new<O, W>(output: O, writer: W, interval: usize) -> Self 
    where
        O: Output + 'static,
        W: std::io::Write + 'static,    
    {
        let output = Box::new(output);
        let writer = Box::new(writer);
        OutputMeta { output, writer, interval }
    }

    pub fn output(&mut self, system: &System, potentials: &Potentials, timestep: usize) {
        let buf = self.output.output(system, potentials, timestep);
        self.writer.write_all(buf.as_bytes()).unwrap();
    }
}
