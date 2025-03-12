// Answer 0

#[test]
fn test_write_u32_zero() {
    let mut dst = [0u8; 4];
    <LE as Endian>::write_u32(0, &mut dst);
}

#[test]
fn test_write_u32_min() {
    let mut dst = [0u8; 4];
    <LE as Endian>::write_u32(1, &mut dst);
}

#[test]
fn test_write_u32_mid_range() {
    let mut dst = [0u8; 4];
    <LE as Endian>::write_u32(2147483648, &mut dst);
}

#[test]
fn test_write_u32_max() {
    let mut dst = [0u8; 4];
    <LE as Endian>::write_u32(u32::MAX, &mut dst);
}

#[test]
fn test_write_u32_boundary() {
    let mut dst = [0u8; 4];
    <LE as Endian>::write_u32(4294967295, &mut dst);
}

