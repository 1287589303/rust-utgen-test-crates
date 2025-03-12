// Answer 0

#[test]
fn test_decode_hex_val_slow_invalid_uppercase() {
    let val: u8 = b'G';
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_special_char() {
    let val: u8 = b'!';
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_zero() {
    let val: u8 = 0;
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_max() {
    let val: u8 = 255;
    let result = decode_hex_val_slow(val);
}

