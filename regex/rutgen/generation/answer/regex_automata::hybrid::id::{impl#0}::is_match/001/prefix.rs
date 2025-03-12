// Answer 0

#[test]
fn test_is_match_mask_set() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_MATCH);
    id.is_match();
}

#[test]
fn test_is_match_mask_set_plus_one() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_MATCH + 1);
    id.is_match();
}

#[test]
fn test_is_match_mask_unset() {
    let id = LazyStateID::new_unchecked(0);
    id.is_match();
}

#[test]
fn test_is_match_mask_unset_minus_one() {
    let id = LazyStateID::new_unchecked(LazyStateID::MASK_MATCH - 1);
    id.is_match();
}

