// Answer 0

#[test]
fn test_from_unit() {
    let u = ();
    let x: Value = u.into();
}

#[test]
#[should_panic]
fn test_from_unit_invalid() {
    // No invalid test case for `()`, as there's only a valid case.
    let u = ();
    let x: Value = u.into();
}

