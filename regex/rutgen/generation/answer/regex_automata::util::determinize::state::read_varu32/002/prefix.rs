// Answer 0

#[test]
fn test_read_varu32_with_boundary_case() {
    let data: &[u8] = &[0b1000_0000; 10];
    let result = read_varu32(data);
}

