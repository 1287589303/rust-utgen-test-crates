// Answer 0

#[test]
fn test_len_invalid_utf8_leading_bytes_1() {
    let byte = 0b1100_0000; // 192
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_2() {
    let byte = 0b1100_0001; // 193
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_3() {
    let byte = 0b1100_0010; // 194
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_4() {
    let byte = 0b1100_0011; // 195
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_5() {
    let byte = 0b1101_1111; // 223
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_6() {
    let byte = 0b1110_0000; // 224
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_7() {
    let byte = 0b1111_0111; // 247
    let result = len(byte);
}

#[test]
fn test_len_invalid_utf8_leading_bytes_8() {
    let byte = 0b1111_1111; // 255
    let result = len(byte);
}

