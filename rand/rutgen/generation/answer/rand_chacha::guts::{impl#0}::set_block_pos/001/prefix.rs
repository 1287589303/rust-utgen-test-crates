// Answer 0

#[test]
fn test_set_block_pos_zero() {
    let key = [0; 32];
    let nonce = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_block_pos(0);
}

#[test]
fn test_set_block_pos_one() {
    let key = [0; 32];
    let nonce = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_block_pos(1);
}

#[test]
fn test_set_block_pos_max() {
    let key = [0; 32];
    let nonce = [0; 12];
    let mut chacha = ChaCha::new(&key, &nonce);
    chacha.set_block_pos(u64::MAX);
}

