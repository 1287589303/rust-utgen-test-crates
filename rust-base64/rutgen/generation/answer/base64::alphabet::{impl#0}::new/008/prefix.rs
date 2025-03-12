// Answer 0

#[test]
fn test_new_invalid_byte_too_low() {
    let alphabet = "\u{00}ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_new_invalid_byte_too_high() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\u{7F}";
    let result = Alphabet::new(alphabet);
}

