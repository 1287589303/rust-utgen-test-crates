// Answer 0

#[test]
fn test_new_alphabet_valid() {
    struct Alphabet;
    impl Alphabet {
        pub const ALPHABET_SIZE: usize = 64;
        pub const PAD_BYTE: u8 = b'=';
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != Self::ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            {
                let mut index = 0;
                while index < Self::ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == Self::PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < Self::ALPHABET_SIZE {
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

    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert!(result.is_ok());
}

#[test]
fn test_new_alphabet_with_minimum_byte() {
    struct Alphabet;
    impl Alphabet {
        pub const ALPHABET_SIZE: usize = 64;
        pub const PAD_BYTE: u8 = b'=';
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != Self::ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            {
                let mut index = 0;
                while index < Self::ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == Self::PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < Self::ALPHABET_SIZE {
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

    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    let result = Alphabet::new("!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~");
    assert!(result.is_ok());
}

#[test]
fn test_new_alphabet_exceeds_size() {
    struct Alphabet;
    impl Alphabet {
        pub const ALPHABET_SIZE: usize = 64;
        pub const PAD_BYTE: u8 = b'=';
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != Self::ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            {
                let mut index = 0;
                while index < Self::ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == Self::PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < Self::ALPHABET_SIZE {
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

    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    assert!(result.is_err());
}

#[test]
fn test_new_alphabet_with_invalid_characters() {
    struct Alphabet;
    impl Alphabet {
        pub const ALPHABET_SIZE: usize = 64;
        pub const PAD_BYTE: u8 = b'=';
        
        pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
            let bytes = alphabet.as_bytes();
            if bytes.len() != Self::ALPHABET_SIZE {
                return Err(ParseAlphabetError::InvalidLength);
            }
            {
                let mut index = 0;
                while index < Self::ALPHABET_SIZE {
                    let byte = bytes[index];

                    if !(byte >= 32_u8 && byte <= 126_u8) {
                        return Err(ParseAlphabetError::UnprintableByte(byte));
                    }
                    if byte == Self::PAD_BYTE {
                        return Err(ParseAlphabetError::ReservedByte(byte));
                    }

                    let mut probe_index = 0;
                    while probe_index < Self::ALPHABET_SIZE {
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

    enum ParseAlphabetError {
        InvalidLength,
        UnprintableByte(u8),
        ReservedByte(u8),
        DuplicatedByte(u8),
    }

    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/-@#&!");
    assert!(result.is_err());
}

