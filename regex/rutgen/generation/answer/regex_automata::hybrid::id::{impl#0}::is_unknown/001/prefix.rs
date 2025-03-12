// Answer 0

#[test]
fn test_is_unknown_boundary_zero() {
    let id = LazyStateID::new_unchecked(0);
    id.is_unknown();
}

#[test]
fn test_is_unknown_boundary_mask() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_UNKNOWN);
    id.is_unknown();
}

#[test]
fn test_is_unknown_boundary_mask_minus_one() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_UNKNOWN - 1);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_one() {
    let id = LazyStateID::new_unchecked(1);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_thirty_one() {
    let id = LazyStateID::new_unchecked(31);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_thirty_two() {
    let id = LazyStateID::new_unchecked(32);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_thirty_three() {
    let id = LazyStateID::new_unchecked(33);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_sixteen() {
    let id = LazyStateID::new_unchecked(16);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_seventeen() {
    let id = LazyStateID::new_unchecked(17);
    id.is_unknown();
}

#[test]
fn test_is_unknown_valid_input_fifteen() {
    let id = LazyStateID::new_unchecked(15);
    id.is_unknown();
}

