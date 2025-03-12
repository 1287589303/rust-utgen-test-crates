// Answer 0

#[test]
fn test_put_u32_valid_input() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_u32(0x12345678);
}

#[test]
fn test_put_u32_boundary_min() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_u32(0);
}

#[test]
fn test_put_u32_boundary_max() {
    let mut buf: Vec<u8> = vec![0; 4];
    buf.put_u32(u32::MAX);
}

#[test] #[should_panic]
fn test_put_u32_insufficient_capacity() {
    let mut buf: Vec<u8> = vec![0; 3];
    buf.put_u32(0x12345678);
}

