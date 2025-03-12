// Answer 0

#[test]
fn test_to_start_with_zero() {
    let lazy_id = LazyStateID::new_unchecked(0);
    let result = lazy_id.to_start();
}

#[test]
fn test_to_start_with_mask_start() {
    let lazy_id = LazyStateID::new_unchecked(LazyStateID::MASK_START);
    let result = lazy_id.to_start();
}

#[test]
fn test_to_start_with_max() {
    let lazy_id = LazyStateID::new_unchecked(LazyStateID::MAX);
    let result = lazy_id.to_start();
}

#[test]
fn test_to_start_with_max_plus_one() {
    let lazy_id = LazyStateID::new_unchecked(LazyStateID::MAX + 1);
    let result = lazy_id.to_start();
}

#[test]
fn test_to_start_with_boundary_value_before_mask_start() {
    let lazy_id = LazyStateID::new_unchecked(LazyStateID::MASK_START - 1);
    let result = lazy_id.to_start();
}

#[test]
fn test_to_start_with_boundary_value_at_mask_dead() {
    let lazy_id = LazyStateID::new_unchecked(LazyStateID::MASK_DEAD);
    let result = lazy_id.to_start();
}

