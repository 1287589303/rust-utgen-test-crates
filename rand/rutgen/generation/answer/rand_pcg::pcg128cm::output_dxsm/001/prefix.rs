// Answer 0

#[test]
fn test_output_dxsm_zero() {
    let state: u128 = 0;
    let result = output_dxsm(state);
}

#[test]
fn test_output_dxsm_one() {
    let state: u128 = 1;
    let result = output_dxsm(state);
}

#[test]
fn test_output_dxsm_max_64_bit() {
    let state: u128 = u64::MAX as u128;
    let result = output_dxsm(state);
}

#[test]
fn test_output_dxsm_min_129_bit() {
    let state: u128 = 1 << 64;
    let result = output_dxsm(state);
}

#[test]
fn test_output_dxsm_max_128_bit() {
    let state: u128 = u128::MAX;
    let result = output_dxsm(state);
}

