use std::fs::File;
use std::io::BufReader;

use vasp_poscar::Poscar;
use velvet_external_data::poscar::import_poscar;
use velvet_test_utils as test_utils;

#[test]
fn import_argon() {
    let file = File::open(test_utils::resources_path("Ar.poscar")).unwrap();
    let reader = BufReader::new(file);
    let poscar = Poscar::from_reader(reader).unwrap();
    let system = import_poscar(&poscar);

    let a0 = 16.922316;
    let alpha = 90.0;
    assert_eq!(system.size, 108);
    assert_eq!(system.cell.a(), a0);
    assert_eq!(system.cell.b(), a0);
    assert_eq!(system.cell.c(), a0);
    assert_eq!(system.cell.alpha(), alpha);
    assert_eq!(system.cell.beta(), alpha);
    assert_eq!(system.cell.gamma(), alpha);
}
