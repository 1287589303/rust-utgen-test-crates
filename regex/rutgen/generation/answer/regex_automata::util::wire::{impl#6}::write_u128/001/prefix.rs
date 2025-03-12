// Answer 0

#[test]
fn test_write_u128_zero() {
    let mut dst = [0u8; 16];
    LE::write_u128(0u128, &mut dst);
}

#[test]
fn test_write_u128_max() {
    let mut dst = [0u8; 16];
    LE::write_u128(u128::MAX, &mut dst);
}

#[test]
fn test_write_u128_midpoint() {
    let mut dst = [0u8; 16];
    LE::write_u128(1 << 127, &mut dst);
}

#[test]
fn test_write_u128_small_value() {
    let mut dst = [0u8; 16];
    LE::write_u128(42u128, &mut dst);
}

#[test]
fn test_write_u128_large_value() {
    let mut dst = [0u8; 16];
    LE::write_u128(123456789012345678901234567890u128, &mut dst);
}

