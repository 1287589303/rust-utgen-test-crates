// Answer 0

#[test]
fn test_to_match_valid_32bit() {
    let state_id = LazyStateID::new_unchecked(0);
    let match_id = state_id.to_match();
}

#[test]
fn test_to_match_valid_31bit() {
    let state_id = LazyStateID::new_unchecked(LazyStateID::MAX);
    let match_id = state_id.to_match();
}

#[test]
fn test_to_match_valid_16bit() {
    let state_id = LazyStateID::new_unchecked(0);
    let match_id = state_id.to_match();
}

#[test]
fn test_to_match_valid_15bit() {
    let state_id = LazyStateID::new_unchecked((1 << 15) - 1);
    let match_id = state_id.to_match();
}

#[test]
fn test_to_match_proxy_id() {
    let state_id = LazyStateID::new_unchecked(5);
    let match_id = state_id.to_match();
}

