// Answer 0

#[test]
fn test_from_str_unchecked_valid() {
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

    let valid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = from_str_unchecked(valid_alphabet);
    assert_eq!(result.symbols, valid_alphabet.as_bytes());
}

#[test]
#[should_panic]
fn test_from_str_unchecked_invalid() {
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

    let invalid_alphabet = "ShortAlphabet"; // Less than ALPHABET_SIZE
    let _result = from_str_unchecked(invalid_alphabet); // This should panic
}

