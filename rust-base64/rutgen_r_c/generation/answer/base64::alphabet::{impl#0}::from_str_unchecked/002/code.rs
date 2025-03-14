// Answer 0

#[test]
fn test_from_str_unchecked_with_full_index() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::from_str_unchecked(alphabet);
    assert_eq!(&result.symbols, alphabet.as_bytes());
}

#[test]
fn test_from_str_unchecked_with_blank_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::from_str_unchecked(alphabet);
    assert_eq!(result.symbols.len(), ALPHABET_SIZE);
}

