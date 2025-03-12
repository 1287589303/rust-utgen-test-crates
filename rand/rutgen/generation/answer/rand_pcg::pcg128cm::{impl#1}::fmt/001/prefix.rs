// Answer 0

#[test]
fn test_lcg128cm_dxsm64_debug_initial_state() {
    let rng = Lcg128CmDxsm64 {
        state: 0,
        increment: 0,
    };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter).unwrap();
}

#[test]
fn test_lcg128cm_dxsm64_debug_modified_state() {
    let rng = Lcg128CmDxsm64 {
        state: 12345678901234567890,
        increment: 98765432109876543210,
    };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter).unwrap();
}

#[test]
fn test_lcg128cm_dxsm64_debug_maximum_values() {
    let rng = Lcg128CmDxsm64 {
        state: u128::MAX,
        increment: u128::MAX,
    };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter).unwrap();
}

#[test]
fn test_lcg128cm_dxsm64_debug_zero_state_increment() {
    let rng = Lcg128CmDxsm64 {
        state: 0,
        increment: u128::MAX,
    };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter).unwrap();
}

#[test]
fn test_lcg128cm_dxsm64_debug_zero_increment_state() {
    let rng = Lcg128CmDxsm64 {
        state: u128::MAX,
        increment: 0,
    };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter).unwrap();
}

