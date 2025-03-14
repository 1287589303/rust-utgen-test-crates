// Answer 0

#[test]
fn test_encode_table_valid_index() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let expected_table: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = encode_table(&alphabet);
    assert_eq!(result, expected_table);
}

#[test]
#[should_panic]
fn test_encode_table_invalid_index() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    // Attempting to access index 64 should panic
    let _result = {
        let encode_table = encode_table(&alphabet);
        encode_table[64]
    };
}

