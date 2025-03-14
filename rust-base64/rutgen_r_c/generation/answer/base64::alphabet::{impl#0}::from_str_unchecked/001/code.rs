// Answer 0

#[test]
fn test_from_str_unchecked_valid() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::from_str_unchecked(alphabet);
    let expected = Alphabet {
        symbols: *alphabet.as_bytes(),
    };
    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid_length() {
    let alphabet = "short"; // length is less than ALPHABET_SIZE
    let _ = Alphabet::from_str_unchecked(alphabet);
}

