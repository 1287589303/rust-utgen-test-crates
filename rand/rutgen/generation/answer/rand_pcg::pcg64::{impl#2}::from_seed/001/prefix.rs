// Answer 0

#[test]
fn test_from_seed_all_zeroes() {
    let seed: [u8; 16] = [0; 16];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_valid_increment() {
    let seed: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 1]; // last byte has a bit ignored
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_boundary_case() {
    let seed: [u8; 16] = [255; 15].iter().cloned().chain(Some(1)).collect::<Vec<u8>>().try_into().unwrap(); // LSB in the last byte is ignored, thus 255...255,1
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_large_odd_value() {
    let seed: [u8; 16] = [
        0, 0, 0, 0, 0, 0, 0, 0, 
        255, 255, 255, 255, 255, 255, 255, 255 // last byte is odd
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
fn test_from_seed_full_range() {
    let seed: [u8; 16] = [
        42, 42, 42, 42, 42, 42, 42, 42,
        0, 0, 0, 0, 0, 0, 0, 3 // odd number
    ];
    let rng = Lcg64Xsh32::from_seed(seed);
}

#[test]
#[should_panic]
fn test_from_seed_invalid_increment() {
    let seed: [u8; 16] = [1; 8].iter().cloned().chain(Some(0)).collect::<Vec<u8>>().try_into().unwrap(); // last byte is 0, so LSB is not odd
    let _rng = Lcg64Xsh32::from_seed(seed);
}

