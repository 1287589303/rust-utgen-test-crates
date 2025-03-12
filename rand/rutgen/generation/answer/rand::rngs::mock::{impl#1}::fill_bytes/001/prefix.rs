// Answer 0

#[test]
fn test_fill_bytes_minimum_size() {
    let mut rng = StepRng { v: 0, a: 1 };
    let mut dst = [0u8; 1];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_small_size() {
    let mut rng = StepRng { v: 1, a: 1 };
    let mut dst = [0u8; 5];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_large_size() {
    let mut rng = StepRng { v: 2, a: 1 };
    let mut dst = [0u8; 1024];
    rng.fill_bytes(&mut dst);
}

