// Answer 0

#[test]
fn test_from_seed_valid_seed() {
    let seed: [u8; 32] = [1; 32]; // valid seed with all bytes non-zero
    let rng = SmallRng::from_seed(seed);
}

#[test]
fn test_from_seed_zero_byte_seed() {
    let seed: [u8; 32] = [0; 32]; // all bytes are zero, should not return a valid state
    let rng = SmallRng::from_seed(seed);
}

#[test]
fn test_from_seed_partial_zero_seed() {
    let seed: [u8; 32] = [0; 31].chain(Some(1)).collect::<Vec<_>>().try_into().unwrap(); // last byte non-zero
    let rng = SmallRng::from_seed(seed);
}

#[test]
fn test_from_seed_non_zero_middle() {
    let mut seed: [u8; 32] = [0; 32];
    seed[16] = 1; // non-zero in the middle
    let rng = SmallRng::from_seed(seed);
}

#[test]
fn test_from_seed_alternating_bytes() {
    let seed: [u8; 32] = [0, 1].iter().cycle().take(32).cloned().collect::<Vec<_>>().try_into().unwrap(); // alternating bytes
    let rng = SmallRng::from_seed(seed);
}

