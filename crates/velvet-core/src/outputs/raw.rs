//! Raw text formatted outputs.

use std::io::Write;

use crate::potentials::collections::Potentials;
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

// This issue: https://github.com/rust-lang/rust/issues/20400
// prevents me from specializing the impl block by the trait's associated type.
// Ideally I will have separate impl blocks for Property<Res=Float> and Property<Res=Vector3<Float>>
// in order to make the formatting more appropriate.

impl<T: Property> RawOutput for T {
    fn output_raw(&self, system: &System, potentials: &Potentials, writer: &mut dyn Write) {
        let res = self.calculate(system, potentials);
        writer
            .write_all(format!("{:#?}: {:#?}\n", self.name(), res).as_bytes())
            .unwrap()
    }
}
