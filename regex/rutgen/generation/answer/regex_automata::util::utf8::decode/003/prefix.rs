// Answer 0

#[test]
fn test_decode_invalid_utf8_two_bytes() {
    let bytes = [0b1100_0001, 0b1000_0000];
    let _result = decode(&bytes);
}

#[test]
fn test_decode_invalid_utf8_three_bytes() {
    let bytes = [0b1110_0001, 0b1000_0000, 0b1000_0000];
    let _result = decode(&bytes);
}

