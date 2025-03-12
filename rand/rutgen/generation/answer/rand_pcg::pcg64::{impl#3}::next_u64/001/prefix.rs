// Answer 0

#[test]
fn test_next_u64_with_minimum_state_increment() {
    let mut rng = Lcg64Xsh32 { state: 0, increment: 0 };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_maximum_state_increment() {
    let mut rng = Lcg64Xsh32 { state: u64::MAX, increment: u64::MAX };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_middle_state_increment() {
    let mut rng = Lcg64Xsh32 { state: u64::MAX / 2, increment: u64::MAX / 2 };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_state_zero_increment_max() {
    let mut rng = Lcg64Xsh32 { state: 0, increment: u64::MAX };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_state_max_increment_zero() {
    let mut rng = Lcg64Xsh32 { state: u64::MAX, increment: 0 };
    let _result = rng.next_u64();
}

