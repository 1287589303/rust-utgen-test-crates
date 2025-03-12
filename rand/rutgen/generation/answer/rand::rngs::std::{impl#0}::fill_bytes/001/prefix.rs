// Answer 0

#[test]
fn test_fill_bytes_small() {
    let mut rng = StdRng(Rng::from_seed([0u8; 32]));
    let mut dst: [u8; 1] = [0; 1];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_medium() {
    let mut rng = StdRng(Rng::from_seed([0u8; 32]));
    let mut dst: [u8; 256] = [0; 256];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_large() {
    let mut rng = StdRng(Rng::from_seed([0u8; 32]));
    let mut dst: [u8; 4096] = [0; 4096];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_boundary() {
    let mut rng = StdRng(Rng::from_seed([0u8; 32]));
    let mut dst: [u8; 4095] = [0; 4095];
    rng.fill_bytes(&mut dst);
    let mut dst: [u8; 2] = [0; 2];
    rng.fill_bytes(&mut dst);
}

