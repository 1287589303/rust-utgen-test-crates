// Answer 0

#[test]
fn test_chacha_new_valid() {
    let key: [u8; 32] = [1; 32];
    let nonce: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let _chacha = ChaCha::new(&key, nonce);
}

#[test]
fn test_chacha_new_min_nonce() {
    let key: [u8; 32] = [255; 32];
    let nonce: &[u8] = &[0];
    let _chacha = ChaCha::new(&key, nonce);
}

#[test]
fn test_chacha_new_max_nonce() {
    let key: [u8; 32] = [0; 32];
    let nonce: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let _chacha = ChaCha::new(&key, nonce);
}

