use crate::outputs::Output;

pub struct Configuration {
    threads: usize,
    outputs: Vec<Box<dyn Output>>,
    output_interval: usize,
    output_filename: &'static str,
}

impl Configuration {
    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn outputs(&self) -> impl Iterator<Item = &Box<dyn Output>> {
        self.outputs.iter()
    }

    pub fn output_interval(&self) -> usize {
        self.output_interval
    }

    pub fn output_filename(&self) -> &'static str {
        self.output_filename
    }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            threads: 1,
            outputs: Vec::new(),
            output_interval: 1,
            output_filename: "velvet.h5",
        }
    }
}

pub struct ConfigurationBuilder {
    threads: Option<usize>,
    outputs: Vec<Box<dyn Output>>,
    output_interval: Option<usize>,
    output_filename: Option<&'static str>,
}

impl ConfigurationBuilder {
    pub fn new() -> ConfigurationBuilder {
        ConfigurationBuilder {
            threads: None,
            outputs: Vec::new(),
            output_interval: None,
            output_filename: None,
        }
    }

    pub fn with_threads(&mut self, threads: usize) -> &mut ConfigurationBuilder {
        self.threads = Some(threads);
        self
    }

    pub fn with_output(&mut self, output: Box<dyn Output>) -> &mut ConfigurationBuilder {
        self.outputs.push(output);
        self
    }

    pub fn with_output_interval(&mut self, interval: usize) -> &mut ConfigurationBuilder {
        self.output_interval = Some(interval);
        self
    }

    pub fn with_output_filename(&mut self, filename: &'static str) -> &mut ConfigurationBuilder {
        self.output_filename = Some(filename);
        self
    }

    pub fn finish(self) -> Configuration {
        let threads = match self.threads {
            Some(t) => t,
            None => 1,
        };

        let outputs = self.outputs;

        let output_interval = match self.output_interval {
            Some(interval) => interval,
            None => 1,
        };

        let output_filename = match self.output_filename {
            Some(filename) => filename,
            None => "velvet.h5",
        };

        Configuration {
            threads,
            outputs,
            output_interval,
            output_filename,
        }
    }
}
