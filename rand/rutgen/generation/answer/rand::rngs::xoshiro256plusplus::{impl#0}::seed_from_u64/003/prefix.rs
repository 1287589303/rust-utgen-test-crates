// Answer 0

#[test]
fn test_seed_from_u64_min_value() {
    let state: u64 = 1;
    let rng = Xoshiro256PlusPlus::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_mid_value() {
    let state: u64 = 0x7FFFFFFFFFFFFFFF;
    let rng = Xoshiro256PlusPlus::seed_from_u64(state);
}

#[test]
fn test_seed_from_u64_max_value() {
    let state: u64 = 0xFFFFFFFFFFFFFFFF;
    let rng = Xoshiro256PlusPlus::seed_from_u64(state);
}

