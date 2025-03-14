// Answer 0

#[test]
fn test_alphabet_creation_valid() {
    struct Alphabet;

    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    #[derive(Debug)]
    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    impl Alphabet {
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }

            {
                let mut index = 0;
                while index < ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < ALPHABET_SIZE {
                        if probe_index == index {
                            probe_index += 1;
                            continue;
                        }

                        let probe_byte = bytes[probe_index];

                        if byte == probe_byte {
                            return Err(ParseAlphabetError::DuplicatedByte(byte));
                        }

                        probe_index += 1;
                    }

                    index += 1;
                }
            }

            Ok(Self {})
        }
    }

    let valid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(valid_alphabet);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_alphabet_creation_invalid_length() {
    struct Alphabet;

    const ALPHABET_SIZE: usize = 64;

    #[derive(Debug)]
    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    impl Alphabet {
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            Ok(Self {})
        }
    }

    let invalid_length_alphabet = "ShortAlphabet";
    let result = Alphabet::new(invalid_length_alphabet);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_alphabet_creation_unprintable_byte() {
    struct Alphabet;

    const ALPHABET_SIZE: usize = 64;

    #[derive(Debug)]
    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    impl Alphabet {
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            // The rest of the implementation...
            Ok(Self {})
        }
    }

    let invalid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00";
    let result = Alphabet::new(invalid_alphabet);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_alphabet_creation_reserved_byte() {
    struct Alphabet;

    const ALPHABET_SIZE: usize = 64;
    const PAD_BYTE: u8 = b'=';

    #[derive(Debug)]
    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    impl Alphabet {
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            // The rest of the implementation...
            Ok(Self {})
        }
    }

    let invalid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=";
    let result = Alphabet::new(invalid_alphabet);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_alphabet_creation_duplicated_byte() {
    struct Alphabet;

    const ALPHABET_SIZE: usize = 64;

    #[derive(Debug)]
    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    impl Alphabet {
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            // The rest of the implementation...
            Ok(Self {})
        }
    }

    let invalid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ12345678";
    let result = Alphabet::new(invalid_alphabet);
    assert!(result.is_err());
}

