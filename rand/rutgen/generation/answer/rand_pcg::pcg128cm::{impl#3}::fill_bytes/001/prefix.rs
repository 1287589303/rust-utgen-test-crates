// Answer 0

#[test]
fn test_fill_bytes_empty() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: 0 };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_single_byte() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: 0 };
    let mut dest = [0u8; 1];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_multiple_bytes() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: 0 };
    let mut dest = [0u8; 16];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_large() {
    let mut rng = Lcg128CmDxsm64 { state: 0, increment: 0 };
    let mut dest = [0u8; 1024];
    rng.fill_bytes(&mut dest);
}

