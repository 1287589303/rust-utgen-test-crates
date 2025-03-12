// Answer 0

#[test]
fn test_fill_bytes_zero_length() {
    let mut rng = Lcg64Xsh32 { state: 0, increment: 0 };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_length() {
    let mut rng = Lcg64Xsh32 { state: 1, increment: 1 };
    let mut dest = [0u8; 1];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_medium_length() {
    let mut rng = Lcg64Xsh32 { state: 2, increment: 2 };
    let mut dest = [0u8; 512];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_large_length() {
    let mut rng = Lcg64Xsh32 { state: 3, increment: 3 };
    let mut dest = [0u8; 1024];
    rng.fill_bytes(&mut dest);
}

