// Answer 0

#[test]
fn test_from_true() {
    let b = true;
    let x: Value = b.into();
}

#[test]
fn test_from_false() {
    let b = false;
    let x: Value = b.into();
}

