// Answer 0

#[test]
fn test_seed_from_u64_zero() {
    let state: u64 = 0;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_one() {
    let state: u64 = 1;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_middle() {
    let state: u64 = 1 << 32;
    let rng = SmallRng::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_max() {
    let state: u64 = u64::MAX;
    let rng = SmallRng::seed_from_u64(state);
}

