// Answer 0

#[test]
fn test_read_varu32_empty() {
    let data: &[u8] = &[];
    let result = read_varu32(data);
}

#[test]
fn test_read_varu32_all_high_bits() {
    let data: &[u8] = &[0b1000_0000, 0b1000_0001, 0b1111_1111];
    let result = read_varu32(data);
}

