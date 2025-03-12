// Answer 0

#[test]
fn test_fill_bytes_zero_size() {
    let mut rng = Xoshiro256PlusPlus { s: [1, 2, 3, 4] };
    let mut dst: [u8; 0] = [];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_small_size() {
    let mut rng = Xoshiro256PlusPlus { s: [5, 6, 7, 8] };
    let mut dst = [0u8; 1];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_medium_size() {
    let mut rng = Xoshiro256PlusPlus { s: [9, 10, 11, 12] };
    let mut dst = [0u8; 512];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_large_size() {
    let mut rng = Xoshiro256PlusPlus { s: [13, 14, 15, 16] };
    let mut dst = [0u8; 1024];
    rng.fill_bytes(&mut dst);
}

