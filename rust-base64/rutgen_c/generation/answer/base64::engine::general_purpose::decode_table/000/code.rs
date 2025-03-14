// Answer 0

#[test]
fn test_decode_table_valid_symbols() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    impl Alphabet for TestAlphabet {
        fn symbols(&self) -> &[u8] {
            &self.symbols
        }
    }

    let valid_symbols = [
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102,
        103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
        117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 43, 47,
    ];

    let test_alphabet = TestAlphabet { symbols: valid_symbols };

    let table = decode_table(&test_alphabet);
    for (index, &symbol) in valid_symbols.iter().enumerate() {
        assert_eq!(table[symbol as usize], index as u8);
    }
}

#[test]
fn test_decode_table_invalid_symbols() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    impl Alphabet for TestAlphabet {
        fn symbols(&self) -> &[u8] {
            &self.symbols
        }
    }

    let invalid_symbols = [
        255, 100, 200, 250, 300, 400, 500, 600, 150, 160, 170, 180, 190, 200, 210, 220,
        230, 240, 250, 255, 240, 230, 220, 210, 200, 190, 180, 170, 160, 150, 140, 130,
        120, 110, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        9, 255, 255,
    ];

    let test_alphabet = TestAlphabet { symbols: invalid_symbols };

    let table = decode_table(&test_alphabet);
    for &symbol in invalid_symbols.iter() {
        assert_eq!(table[symbol as usize], INVALID_VALUE);
    }
}

