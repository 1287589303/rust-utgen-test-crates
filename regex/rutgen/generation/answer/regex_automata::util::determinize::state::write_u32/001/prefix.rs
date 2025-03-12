// Answer 0

#[test]
fn test_write_u32_empty_vec() {
    let mut dst = Vec::new();
    let n: u32 = 0;
    write_u32(&mut dst, n);
}

#[test]
fn test_write_u32_small_value() {
    let mut dst = Vec::new();
    let n: u32 = 1;
    write_u32(&mut dst, n);
}

#[test]
fn test_write_u32_mid_range_value() {
    let mut dst = Vec::new();
    let n: u32 = 2_147_483_647; // Max of i32
    write_u32(&mut dst, n);
}

#[test]
fn test_write_u32_large_value() {
    let mut dst = Vec::new();
    let n: u32 = 4_294_967_295; // Max of u32
    write_u32(&mut dst, n);
}

#[test]
fn test_write_u32_non_empty_vec() {
    let mut dst = vec![0; 4]; // Pre-fill with some bytes
    let n: u32 = 12345678;
    write_u32(&mut dst, n);
}

#[test]
fn test_write_u32_multiple_writes() {
    let mut dst = Vec::new();
    let values = vec![0, 123, 456, 789, 1_000_000, 4_294_967_295];
    for &n in &values {
        write_u32(&mut dst, n);
    }
}

