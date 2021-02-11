use std::fs::File;
use std::io::BufReader;

use velvet_external_data::poscar::load_poscar;
use velvet_test_utils as test_utils;

#[test]
fn argon() {
    let file = File::open(test_utils::resources_path("argon.poscar")).unwrap();
    let reader = BufReader::new(file);
    let sys = load_poscar(reader);

    println!("{:?}", sys.cell());

    let a0 = 21.152895;
    assert_eq!(sys.size(), 108);
    assert_eq!(sys.cell().a(), a0);
    assert_eq!(sys.cell().b(), a0);
    assert_eq!(sys.cell().c(), a0);
    assert_eq!(sys.cell().alpha(), 90.0);
    assert_eq!(sys.cell().beta(), 90.0);
    assert_eq!(sys.cell().gamma(), 90.0);
}
