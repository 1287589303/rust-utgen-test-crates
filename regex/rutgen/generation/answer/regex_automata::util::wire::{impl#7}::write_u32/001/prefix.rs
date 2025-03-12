// Answer 0

#[test]
fn test_write_u32_min() {
    let mut dst = [0u8; 4];
    let n: u32 = 0;
    BE::write_u32(n, &mut dst);
}

#[test]
fn test_write_u32_max() {
    let mut dst = [0u8; 4];
    let n: u32 = 4294967295;
    BE::write_u32(n, &mut dst);
}

#[test]
fn test_write_u32_mid() {
    let mut dst = [0u8; 4];
    let n: u32 = 2147483648;
    BE::write_u32(n, &mut dst);
}

#[test]
fn test_write_u32_boundary_lower() {
    let mut dst = [0u8; 4];
    let n: u32 = 1;
    BE::write_u32(n, &mut dst);
}

#[test]
fn test_write_u32_boundary_upper() {
    let mut dst = [0u8; 4];
    let n: u32 = 4294967294;
    BE::write_u32(n, &mut dst);
}

