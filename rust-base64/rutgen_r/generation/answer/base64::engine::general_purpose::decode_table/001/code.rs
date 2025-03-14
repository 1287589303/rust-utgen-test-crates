// Answer 0

#[test]
fn test_decode_table_valid() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    const INVALID_VALUE: u8 = 255;

    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", // A standard base64 alphabet
    };

    let result = decode_table(&alphabet);
    let expected: [u8; 256] = {
        let mut arr = [INVALID_VALUE; 256];
        for (i, &symbol) in alphabet.symbols.iter().enumerate() {
            arr[symbol as usize] = i as u8;
        }
        arr
    };

    assert_eq!(result, expected);
}

#[test]
fn test_decode_table_invalid() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    const INVALID_VALUE: u8 = 255;

    // A valid alphabet for this context
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let result = decode_table(&alphabet);

    // Ensure that all values outside the base64 range are INVALID_VALUE
    for i in 0..256 {
        if i < 0x20 || i >= 0x60 {
            assert_eq!(result[i], INVALID_VALUE);
        }
    }
}

