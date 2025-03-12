// Answer 0

#[test]
fn test_to_unknown_with_zero() {
    let id = LazyStateID::new_unchecked(0);
    let _result = id.to_unknown();
}

#[test]
fn test_to_unknown_with_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX as usize);
    let _result = id.to_unknown();
}

#[test]
fn test_to_unknown_with_mid_range() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX as usize / 2);
    let _result = id.to_unknown();
}

#[test]
fn test_to_unknown_with_boundary_low() {
    let id = LazyStateID::new_unchecked(1);
    let _result = id.to_unknown();
}

#[test]
fn test_to_unknown_with_boundary_high() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX as usize - 1);
    let _result = id.to_unknown();
}

