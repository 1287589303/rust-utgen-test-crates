// Answer 0

#[test]
fn test_from_seed_all_zeros() {
    let seed: [u8; 32] = [0; 32];
    let rng = StdRng::from_seed(seed);
}

#[test]
fn test_from_seed_all_ones() {
    let seed: [u8; 32] = [1; 32];
    let rng = StdRng::from_seed(seed);
}

#[test]
fn test_from_seed_all_twos() {
    let seed: [u8; 32] = [2; 32];
    let rng = StdRng::from_seed(seed);
}

#[test]
fn test_from_seed_boundary_values() {
    let seed_min: [u8; 32] = [0; 32];
    let seed_max: [u8; 32] = [255; 32];
    
    let rng_min = StdRng::from_seed(seed_min);
    let rng_max = StdRng::from_seed(seed_max);
}

#[test]
fn test_from_seed_incrementing_values() {
    let seed: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 
                          16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
    let rng = StdRng::from_seed(seed);
}

#[test]
fn test_from_seed_half_and_half() {
    let seed: [u8; 32] = [128; 16].iter().chain([64; 16].iter()).cloned().collect::<Vec<u8>>().try_into().unwrap();
    let rng = StdRng::from_seed(seed);
}

