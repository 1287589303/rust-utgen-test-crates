// Answer 0

#[test]
fn test_from_zero() {
    let result = Ref::from(0);
}

#[test]
fn test_from_one() {
    let result = Ref::from(1);
}

#[test]
fn test_from_max() {
    let result = Ref::from(usize::MAX);
}

#[test]
fn test_from_large_value() {
    let result = Ref::from(123456789);
}

