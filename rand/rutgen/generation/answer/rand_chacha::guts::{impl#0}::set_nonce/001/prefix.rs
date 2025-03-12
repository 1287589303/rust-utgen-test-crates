// Answer 0

#[test]
fn test_set_nonce_zero() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(0);
}

#[test]
fn test_set_nonce_one() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(1);
}

#[test]
fn test_set_nonce_two() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(2);
}

#[test]
fn test_set_nonce_max() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(18446744073709551615);
}

#[test]
fn test_set_nonce_random_100() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(100);
}

#[test]
fn test_set_nonce_random_123456789() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(123456789);
}

#[test]
fn test_set_nonce_random_4294967295() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[];
    let mut chacha = ChaCha::new(&key, nonce);
    chacha.set_nonce(4294967295);
}

