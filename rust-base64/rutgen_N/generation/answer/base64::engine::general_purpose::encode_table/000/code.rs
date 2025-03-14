// Answer 0

#[test]
fn test_encode_table() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let result = encode_table(&alphabet);

    let expected: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    assert_eq!(result, expected);
}

#[test]
fn test_encode_table_empty() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: [0; 64],
    };

    let result = encode_table(&alphabet);

    let expected: [u8; 64] = [0; 64];
    assert_eq!(result, expected);
}

