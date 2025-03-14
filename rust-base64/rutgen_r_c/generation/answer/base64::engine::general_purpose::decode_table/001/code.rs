// Answer 0

#[test]
fn test_decode_table_with_valid_alphabet() {
    // Construct a valid alphabet instance
    let valid_symbols = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];
    let alphabet = Alphabet { symbols: valid_symbols };

    let result = decode_table(&alphabet);

    // Check that the decode_table correctly maps the symbols to their index values
    for i in 0..64 {
        assert_eq!(result[alphabet.symbols[i] as usize], i as u8);
    }
    // Check that invalid characters map to INVALID_VALUE
    for i in 64..256 {
        assert_eq!(result[i], INVALID_VALUE);
    }
}

#[test]
fn test_decode_table_with_empty_alphabet() {
    // Construct an alphabet with empty symbols (beyond valid range)
    let empty_symbols = [0; 64];
    let alphabet = Alphabet { symbols: empty_symbols };

    let result = decode_table(&alphabet);

    // Since all symbols are invalid, the entire decode_table should remain as INVALID_VALUE
    for i in 0..256 {
        assert_eq!(result[i], INVALID_VALUE);
    }
}

