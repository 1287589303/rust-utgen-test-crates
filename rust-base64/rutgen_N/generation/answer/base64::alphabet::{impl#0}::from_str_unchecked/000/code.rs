// Answer 0

#[test]
fn test_from_str_unchecked_valid_input() {
    const ALPHABET_SIZE: usize = 64;
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

    let valid_input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = from_str_unchecked(valid_input);
    assert_eq!(alphabet.symbols, valid_input.as_bytes());
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid_input_length() {
    const ALPHABET_SIZE: usize = 64;
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

    let invalid_input = "Short input";
    let _ = from_str_unchecked(invalid_input);
}

