// Answer 0

#[test]
fn test_h1_zero() {
    let result = h1(0);
}

#[test]
fn test_h1_one() {
    let result = h1(1);
}

#[test]
fn test_h1_max() {
    let result = h1(u64::MAX);
}

#[test]
fn test_h1_max_minus_one() {
    let result = h1(u64::MAX - 1);
}

#[test]
fn test_h1_u32_max() {
    let result = h1(u32::MAX as u64);
}

#[test]
fn test_h1_u32_max_plus_one() {
    let result = h1((u32::MAX as u64) + 1);
}

