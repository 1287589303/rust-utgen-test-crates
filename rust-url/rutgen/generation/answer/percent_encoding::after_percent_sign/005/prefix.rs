// Answer 0

#[test]
fn test_after_percent_sign_valid_hex_lowercase() {
    let input: &[u8] = b"1a";
    let mut iter = input.iter();
    let result = after_percent_sign(&mut iter);
}

#[test]
fn test_after_percent_sign_valid_hex_uppercase() {
    let input: &[u8] = b"0F";
    let mut iter = input.iter();
    let result = after_percent_sign(&mut iter);
}

#[test]
fn test_after_percent_sign_valid_hex_mixedcase() {
    let input: &[u8] = b"3b";
    let mut iter = input.iter();
    let result = after_percent_sign(&mut iter);
}

#[test]
fn test_after_percent_sign_valid_hex_zero() {
    let input: &[u8] = b"00";
    let mut iter = input.iter();
    let result = after_percent_sign(&mut iter);
}

#[test]
fn test_after_percent_sign_valid_hex_maximum() {
    let input: &[u8] = b"ff";
    let mut iter = input.iter();
    let result = after_percent_sign(&mut iter);
}

