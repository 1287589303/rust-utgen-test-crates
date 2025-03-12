// Answer 0

#[test]
fn test_from_seed_zero_seed() {
    let seed: [u8; 32] = [0; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_non_zero_seed() {
    let seed: [u8; 32] = [1; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_alternating_seed() {
    let seed: [u8; 32] = [0; 31].iter().chain([1u8]).copied().collect::<Vec<u8>>().try_into().unwrap();
    let rng = Xoshiro256PlusPlus::from_seed(seed);
}

#[test]
fn test_from_seed_boundary_cases() {
    let seed_all_max: [u8; 32] = [255; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed_all_max);

    let seed_half_zero: [u8; 32] = [0; 16].iter().chain([1; 16]).copied().collect::<Vec<u8>>().try_into().unwrap();
    let rng_half_zero = Xoshiro256PlusPlus::from_seed(seed_half_zero);
    
    let seed_one_zero: [u8; 32] = [0; 31].iter().chain([1u8]).copied().collect::<Vec<u8>>().try_into().unwrap();
    let rng_one_zero = Xoshiro256PlusPlus::from_seed(seed_one_zero);
}

