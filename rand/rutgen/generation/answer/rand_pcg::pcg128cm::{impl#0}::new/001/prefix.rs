// Answer 0

#[test]
fn test_new_with_min_state_and_min_stream() {
    let state: u128 = 0;
    let stream: u128 = 0;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_with_min_state_and_max_stream() {
    let state: u128 = 0;
    let stream: u128 = u128::MAX;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_with_max_state_and_min_stream() {
    let state: u128 = u128::MAX;
    let stream: u128 = 0;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_with_max_state_and_max_stream() {
    let state: u128 = u128::MAX;
    let stream: u128 = u128::MAX;
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_with_random_valid_stream() {
    let state: u128 = 0x1234567890abcdef1234567890abcdef;
    let stream: u128 = 0xabcdefabcdefabcdefabcdefabcdefabcdef >> 1; // ensure highest bit is discarded
    let rng = Lcg128CmDxsm64::new(state, stream);
}

#[test]
fn test_new_with_odd_stream_value() {
    let state: u128 = 0xdeadbeefcafef00dd15ea5e5;
    let stream: u128 = 0x7fffffffffffffff; // odd stream value after discarding highest bit
    let rng = Lcg128CmDxsm64::new(state, stream);
} 

#[test]
fn test_new_with_increment_calculation() {
    let state: u128 = 0x1;
    let stream: u128 = 0x2; // this will cause increment to be odd after shifting
    let rng = Lcg128CmDxsm64::new(state, stream);
}

