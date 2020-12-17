//! Convert external file formats into Velvet data structures.

mod poscar;
pub use poscar::load_poscar;

#[allow(dead_code)]
fn test_resources_path(filename: &str) -> String {
    format!(
        "{}/../../resources/test/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    )
}
