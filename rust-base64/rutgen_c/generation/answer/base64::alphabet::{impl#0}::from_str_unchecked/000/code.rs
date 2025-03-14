// Answer 0

#[test]
fn test_from_str_unchecked() {
    let valid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = Alphabet::from_str_unchecked(valid_alphabet);
    assert_eq!(alphabet.symbols, valid_alphabet.as_bytes());
}

#[test]
fn test_from_str_unchecked_multiple_characters() {
    let valid_alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/";
    let alphabet = Alphabet::from_str_unchecked(valid_alphabet);
    assert_eq!(alphabet.symbols, valid_alphabet.as_bytes());
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid_length() {
    let invalid_alphabet = "short";
    let _alphabet = Alphabet::from_str_unchecked(invalid_alphabet);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_reserved_byte() {
    let invalid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // contains PAD_BYTE which is typically the `=` character
    let _alphabet = Alphabet::from_str_unchecked(invalid_alphabet);
}

#[test]
fn test_from_str_unchecked_boundary_characters() {
    let valid_alphabet = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]_^`abcdefghijklmnopqrstuvwxyz{|}~";
    let alphabet = Alphabet::from_str_unchecked(valid_alphabet);
    assert_eq!(alphabet.symbols, valid_alphabet.as_bytes());
}

