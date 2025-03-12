// Answer 0

#[test]
fn test_is_tagged_below_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX);
    id.is_tagged();
}

#[test]
fn test_is_tagged_at_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX);
    id.is_tagged();
}

#[test]
fn test_is_tagged_above_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX + 1);
    id.is_tagged();
}

#[test]
fn test_is_tagged_zero() {
    let id = LazyStateID::new_unchecked(0);
    id.is_tagged();
}

