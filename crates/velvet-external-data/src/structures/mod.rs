pub mod poscar;

use std::fs::File;
use std::io::Write;

use velvet_core::system::System;

pub trait StructureFormat {
    fn parse_system_from_file<T: AsRef<str>>(&self, filename: T) -> System {
        let file = File::open(filename.as_ref()).unwrap();
        self.parse_system_from_reader(file)
    }

    fn parse_system_from_reader<T: std::io::Read>(&self, reader: T) -> System;

    fn write_file_from_system<T: AsRef<str>>(&self, system: &System, filename: T) {
        let s = self.write_str_from_system(system);
        let mut file = File::create(filename.as_ref()).unwrap();
        file.write_all(s.as_bytes()).unwrap()
    }

    fn write_str_from_system(&self, system: &System) -> &str;
}
