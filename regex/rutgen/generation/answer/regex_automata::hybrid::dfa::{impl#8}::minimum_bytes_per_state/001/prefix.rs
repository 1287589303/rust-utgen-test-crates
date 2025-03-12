// Answer 0

#[test]
fn test_minimum_bytes_per_state_zero() {
    let config = Config::new().minimum_bytes_per_state(Some(0));
}

#[test]
fn test_minimum_bytes_per_state_one() {
    let config = Config::new().minimum_bytes_per_state(Some(1));
}

#[test]
fn test_minimum_bytes_per_state_max() {
    let config = Config::new().minimum_bytes_per_state(Some(usize::MAX));
}

#[test]
fn test_minimum_bytes_per_state_none() {
    let config = Config::new().minimum_bytes_per_state(None);
}

