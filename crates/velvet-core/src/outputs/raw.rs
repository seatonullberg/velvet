//! Raw text formatted outputs.

use std::io::Write;

use crate::potentials::collections::Potentials;
use crate::properties::energy::{KineticEnergy, PairEnergy, PotentialEnergy, TotalEnergy};
use crate::properties::forces::Forces;
use crate::properties::temperature::Temperature;
use crate::properties::Property;
use crate::system::System;

/// Shared behavior to write a simulation result as raw text.
pub trait RawOutput {
    /// Writes the raw text formatted output.
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write);
}

pub struct RawOutputGroup {
    pub destination: Box<dyn Write>,
    pub interval: usize,
    pub outputs: Vec<Box<dyn RawOutput>>,
}

pub struct RawOutputGroupBuilder {
    destination: Box<dyn Write>,
    interval: usize,
    outputs: Vec<Box<dyn RawOutput>>,
}

impl RawOutputGroupBuilder {
    pub fn new() -> RawOutputGroupBuilder {
        RawOutputGroupBuilder {
            destination: Box::new(std::io::stderr()),
            interval: 1,
            outputs: Vec::new(),
        }
    }

    pub fn destination<T: Write + 'static>(mut self, destination: T) -> RawOutputGroupBuilder {
        self.destination = Box::new(destination);
        self
    }

    pub fn interval(mut self, interval: usize) -> RawOutputGroupBuilder {
        self.interval = interval;
        self
    }

    pub fn output<T: RawOutput + 'static>(mut self, output: T) -> RawOutputGroupBuilder {
        self.outputs.push(Box::new(output));
        self
    }

    pub fn build(self) -> RawOutputGroup {
        RawOutputGroup {
            destination: self.destination,
            interval: self.interval,
            outputs: self.outputs,
        }
    }
}

impl RawOutput for Forces {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let forces = self.calculate(system, potentials);
        writer
            .write_all(format!("Forces: {:#?}\n", forces).as_bytes())
            .unwrap()
    }
}

impl RawOutput for KineticEnergy {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let ke = self.calculate(system, potentials);
        writer
            .write_all(format!("Kinetic Energy: {:#?}\n", ke).as_bytes())
            .unwrap()
    }
}

impl RawOutput for PotentialEnergy {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let pe = self.calculate(system, potentials);
        writer
            .write_all(format!("Potential Energy: {:#?}\n", pe).as_bytes())
            .unwrap()
    }
}

impl RawOutput for TotalEnergy {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let etotal = self.calculate(system, potentials);
        writer
            .write_all(format!("Total Energy: {:#?}\n", etotal).as_bytes())
            .unwrap()
    }
}

impl RawOutput for PairEnergy {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let epair = self.calculate(system, potentials);
        writer
            .write_all(format!("Pair Energy: {:#?}\n", epair).as_bytes())
            .unwrap()
    }
}

impl RawOutput for Temperature {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let temp = self.calculate(system, potentials);
        writer
            .write_all(format!("Temperature: {:#?}\n", temp).as_bytes())
            .unwrap()
    }
}
