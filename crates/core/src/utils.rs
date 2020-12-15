use crate::potentials::Potentials;
use crate::system::System;
use ron;
use std::fs::File;

pub fn test_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}

pub fn load_test_system(name: &str) -> System {
    let mut path = test_path(name);
    path.push_str(".sys.velvet");
    let f = File::open(&path).expect("failed to open test file");
    let sys: System = ron::de::from_reader(f).unwrap();
    sys
}

pub fn load_test_potentials(name: &str) -> Potentials {
    let mut path = test_path(name);
    path.push_str(".pot.velvet");
    let f = File::open(&path).expect("failed to open test file");
    let pots: Potentials = ron::de::from_reader(f).unwrap();
    pots
}
