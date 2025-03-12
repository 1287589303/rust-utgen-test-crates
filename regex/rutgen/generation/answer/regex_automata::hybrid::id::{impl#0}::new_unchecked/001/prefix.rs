// Answer 0

#[test]
fn test_new_unchecked_min_value() {
    let id = 0;
    let result = LazyStateID::new_unchecked(id);
}

#[test]
fn test_new_unchecked_max_value() {
    let id = LazyStateID::MAX;
    let result = LazyStateID::new_unchecked(id);
}

#[test]
fn test_new_unchecked_boundary_value() {
    let id = LazyStateID::MAX - 1;
    let result = LazyStateID::new_unchecked(id);
}

#[test]
fn test_new_unchecked_mid_value() {
    let id = LazyStateID::MAX / 2;
    let result = LazyStateID::new_unchecked(id);
}

