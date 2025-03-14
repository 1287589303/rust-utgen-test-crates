// Answer 0

#[test]
fn test_new_alphabet_invalid_length() {
    let short_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567"; // 63 bytes
    let result = Alphabet::new(short_alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_unprintable_byte() {
    struct Bounds;

    impl Bounds {
        fn alphabet_with_unprintable() -> &'static str {
            // 64 bytes, first byte is unprintable (byte 31)
            "ABCDEFGHIJKLMNOPQRSTUVWXYPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" // 31 is unprintable
        }
    }

    let result = Alphabet::new(Bounds::alphabet_with_unprintable());
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(31)));
}

#[test]
fn test_new_alphabet_with_del_byte() {
    struct Bounds;

    impl Bounds {
        fn alphabet_with_del() -> &'static str {
            // 64 bytes, last byte is DEL (byte 127)
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/^" // includes byte 127
        }
    }

    let result = Alphabet::new(Bounds::alphabet_with_del_byte());
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(127)));
}

