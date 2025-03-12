// Answer 0

#[test]
fn test_fill_bytes_empty_slice() {
    let mut rng = SmallRng(Xoshiro256PlusPlus { s: [0; 4] });
    let mut dest: &mut [u8] = &mut [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_slice() {
    let mut rng = SmallRng(Xoshiro256PlusPlus { s: [0; 4] });
    let mut dest = [0u8; 1];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_medium_slice() {
    let mut rng = SmallRng(Xoshiro256PlusPlus { s: [0; 4] });
    let mut dest = [0u8; 128];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_full_capacity_slice() {
    let mut rng = SmallRng(Xoshiro256PlusPlus { s: [0; 4] });
    let mut dest = [0u8; 256];
    rng.fill_bytes(&mut dest);
}

