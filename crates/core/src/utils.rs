/// Loads the specified test system into a System object.
#[macro_export]
macro_rules! load_test_system {
    ($filename:literal) => {{
        let path = format!(
            "{}/../../resources/test/{}.velvet.sys",
            env!("CARGO_MANIFEST_DIR"),
            $filename
        );
        let f = std::fs::File::open(&path).expect("failed to open test file");
        let sys: crate::system::System = ron::de::from_reader(f).unwrap();
        sys
    }};
}

/// Loads the specified test potentials into a Potentials object.
#[macro_export]
macro_rules! load_test_potentials {
    ($filename:literal) => {{
        let path = format!(
            "{}/../../resources/test/{}.velvet.pot",
            env!("CARGO_MANIFEST_DIR"),
            $filename
        );
        let f = std::fs::File::open(&path).expect("failed to open test file");
        let pots: crate::potentials::Potentials = ron::de::from_reader(f).unwrap();
        pots
    }};
}
