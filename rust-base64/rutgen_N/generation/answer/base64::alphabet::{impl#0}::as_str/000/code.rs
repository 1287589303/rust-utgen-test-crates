// Answer 0

#[test]
fn test_alphabet_as_str() {
    struct Alphabet {
        symbols: Vec<u8>,
    }

    impl Alphabet {
        pub fn new(symbols: Vec<u8>) -> Self {
            Self { symbols }
        }

        pub fn as_str(&self) -> &str {
            core::str::from_utf8(&self.symbols).unwrap()
        }
    }

    let alphabet = Alphabet::new(vec![b'A', b'B', b'C']);
    assert_eq!(alphabet.as_str(), "ABC");
}

#[test]
#[should_panic]
fn test_alphabet_as_str_invalid_utf8() {
    struct Alphabet {
        symbols: Vec<u8>,
    }

    impl Alphabet {
        pub fn new(symbols: Vec<u8>) -> Self {
            Self { symbols }
        }

        pub fn as_str(&self) -> &str {
            core::str::from_utf8(&self.symbols).unwrap()
        }
    }

    let alphabet = Alphabet::new(vec![0, 159, 146, 150]); // Invalid UTF-8 sequence
    alphabet.as_str();
}

