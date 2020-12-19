//! Convert external file formats into Velvet data structures.

pub mod poscar;

pub mod prelude {
    pub use super::poscar::*;
}

#[allow(dead_code)]
fn test_resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
