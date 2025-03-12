// Answer 0

#[test]
fn test_len_case_valid_two_byte() {
    let byte: u8 = 0b1101_1111; // This tests the case where byte ≤ 0b1101_1111 and matches the conditions
    len(byte);
}

#[test]
fn test_len_case_invalid_leading_byte() {
    let byte: u8 = 0b1000_0000; // This tests the case where the byte makes byte & 0b1100_0000 == 0b1000_0000 true
    len(byte);
}

#[test]
fn test_len_case_boundary_condition() {
    let byte: u8 = 0b1101_1111; // This tests the highest valid UTF-8 leading byte for two bytes
    len(byte);
}

