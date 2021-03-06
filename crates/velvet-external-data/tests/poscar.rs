use velvet_test_utils as test_utils;

#[test]
fn import_argon() {
    let system = test_utils::argon_system();

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
