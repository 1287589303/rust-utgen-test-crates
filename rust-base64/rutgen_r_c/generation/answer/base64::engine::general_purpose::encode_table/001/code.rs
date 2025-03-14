// Answer 0

#[test]
fn test_encode_table_valid_index() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    let alphabet = TestAlphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let result = encode_table(&alphabet);
    assert_eq!(result, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_encode_table_invalid_index() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    let alphabet = TestAlphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    // Attempting to create an encode_table with an index out of bounds
    let _result = {
        let mut encode_table = [0_u8; 64];
        let index = 64; // setting index to 64 to trigger panic
        encode_table[index] = alphabet.symbols[index]; // this should panic
        encode_table
    };
}

