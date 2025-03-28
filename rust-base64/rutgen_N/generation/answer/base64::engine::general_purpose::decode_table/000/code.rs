// Answer 0

#[test]
fn test_decode_table_with_valid_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    const VALID_SYMBOLS: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];

    let alphabet = Alphabet { symbols: VALID_SYMBOLS };
    let table = decode_table(&alphabet);

    for (i, &symbol) in alphabet.symbols.iter().enumerate() {
        assert_eq!(table[symbol as usize], i as u8);
    }

    for i in 0..=255 {
        if !alphabet.symbols.contains(&(i as u8)) {
            assert_eq!(table[i], INVALID_VALUE);
        }
    }
}

#[test]
fn test_decode_table_with_empty_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    const EMPTY_SYMBOLS: [u8; 64] = [0; 64];

    let alphabet = Alphabet { symbols: EMPTY_SYMBOLS };
    let table = decode_table(&alphabet);

    for i in 0..=255 {
        assert_eq!(table[i], INVALID_VALUE);
    }
}

