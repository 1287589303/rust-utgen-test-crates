// Answer 0

#[test]
fn test_as_usize_untagged_valid() {
    let id = LazyStateID::new_unchecked(0);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_half_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX / 2);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_near_max() {
    let id = LazyStateID::new_unchecked(LazyStateID::MAX - 1);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_unknown() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_UNKNOWN);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_dead() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_DEAD);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_quit() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_QUIT);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_start() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_START);
    let result = id.as_usize_untagged();
}

#[test]
fn test_as_usize_untagged_match() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_MATCH);
    let result = id.as_usize_untagged();
}

