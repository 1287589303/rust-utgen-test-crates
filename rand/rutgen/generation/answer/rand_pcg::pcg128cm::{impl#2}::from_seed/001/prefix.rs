// Answer 0

#[test]
fn test_from_seed_all_zeroes() {
    let seed: [u8; 32] = [0; 32];
    let rng = Lcg128CmDxsm64::from_seed(seed);
}

#[test]
fn test_from_seed_all_ones() {
    let seed: [u8; 32] = [255; 32];
    let rng = Lcg128CmDxsm64::from_seed(seed);
}

#[test]
fn test_from_seed_pattern_1() {
    let seed: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
    let rng = Lcg128CmDxsm64::from_seed(seed);
}

#[test]
fn test_from_seed_boundary_pattern() {
    let seed: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
    let rng = Lcg128CmDxsm64::from_seed(seed);
}

#[test]
fn test_from_seed_single_bit_set() {
    let seed: [u8; 32] = [0; 31];
    let seed: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let rng = Lcg128CmDxsm64::from_seed(seed);
}

#[test]
fn test_from_seed_increment_odd() {
    let seed: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0];
    let rng = Lcg128CmDxsm64::from_seed(seed);
}

