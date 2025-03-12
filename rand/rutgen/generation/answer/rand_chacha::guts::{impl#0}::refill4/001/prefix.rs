// Answer 0

#[test]
fn test_refill4_with_max_drounds() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [1; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let drounds: u32 = u32::MAX;
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(drounds, &mut output);
}

#[test]
fn test_refill4_with_zero_drounds() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 1] = [1];
    let mut chacha = ChaCha::new(&key, &nonce);
    let drounds: u32 = 0;
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(drounds, &mut output);
}

#[test]
fn test_refill4_with_boundary_drounds() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [2; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let drounds: u32 = 1;
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(drounds, &mut output);
}

#[test]
fn test_refill4_with_small_nonce() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 1] = [3];
    let mut chacha = ChaCha::new(&key, &nonce);
    let drounds: u32 = 1;
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(drounds, &mut output);
}

#[test]
fn test_refill4_with_max_nonce_length() {
    let key: [u8; 32] = [0; 32];
    let nonce: [u8; 12] = [4; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    let drounds: u32 = 1;
    let mut output: [u32; BUFSZ] = [0; BUFSZ];
    chacha.refill4(drounds, &mut output);
}

