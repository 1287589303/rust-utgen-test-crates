// Answer 0

#[test]
fn test_encode_table_standard_alphabet() {
    let alphabet = Alphabet {
        symbols: [b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
                  b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
                  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
                  b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
                  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
                  b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
                  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
                  b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/'],
    };
    
    let expected_table = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];

    let table = encode_table(&alphabet);
    assert_eq!(table, expected_table);
}

#[test]
fn test_encode_table_custom_alphabet() {
    let custom_alphabet = Alphabet {
        symbols: [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                  b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
                  b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
                  b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
                  b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
                  b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
                  b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
                  b'u', b'v', b'w', b'x', b'y', b'z', b'+', b'/'],
    };
    
    let expected_table = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
        b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
        b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
        b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
        b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z', b'+', b'/',
    ];

    let table = encode_table(&custom_alphabet);
    assert_eq!(table, expected_table);
} 

#[test]
fn test_encode_table_empty_alphabet() {
    let alphabet = Alphabet {
        symbols: [0; 64],
    };
    
    let expected_table = [0; 64];
    let table = encode_table(&alphabet);
    assert_eq!(table, expected_table);
}

