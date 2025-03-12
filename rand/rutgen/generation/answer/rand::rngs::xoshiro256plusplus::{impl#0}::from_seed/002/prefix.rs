// Answer 0

#[test]
fn test_from_seed_non_zero_seed() {
    let seed: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_another_non_zero_seed() {
    let seed: [u8; 32] = [255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_mixed_non_zero_seed() {
    let seed: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_all_non_zero_seed_end() {
    let seed: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

