// Answer 0

#[test]
fn test_decode_four_hex_digits_all_zeroes() {
    let result = decode_four_hex_digits(0x00, 0x00, 0x00, 0x00);
}

#[test]
fn test_decode_four_hex_digits_large_values() {
    let result = decode_four_hex_digits(254, 254, 254, 254);
}

