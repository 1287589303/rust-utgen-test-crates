// Answer 0

#[test]
fn test_from_str_unchecked_boundary_case() {
    const ALPHABET_SIZE: usize = 64; // Assuming ALPHABET_SIZE is defined as 64
    struct Alphabet {
        symbols: [u8; ALPHABET_SIZE],
    }

    const fn from_str_unchecked(alphabet: &str) -> Alphabet {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Alphabet { symbols }
    }

    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // 64 characters
    let result = from_str_unchecked(alphabet_str);
    assert_eq!(result.symbols, alphabet_str.as_bytes());
}

#[test]
#[should_panic]
fn test_from_str_unchecked_out_of_bounds() {
    const ALPHABET_SIZE: usize = 64; // Assuming ALPHABET_SIZE is defined as 64
    struct Alphabet {
        symbols: [u8; ALPHABET_SIZE],
    }

    const fn from_str_unchecked(alphabet: &str) -> Alphabet {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Alphabet { symbols }
    }

    let alphabet_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // 26 characters, should panic
    let _ = from_str_unchecked(alphabet_str);
}

