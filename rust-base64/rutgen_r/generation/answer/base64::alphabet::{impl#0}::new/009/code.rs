// Answer 0


#[derive(Debug)]
struct ParseAlphabetError {
    message: &'static str,
}

impl ParseAlphabetError {
    const InvalidLength: Self = Self { message: "Invalid length" };
    const UnprintableByte: fn(u8) -> Self = |byte| Self { message: "Unprintable byte" };
    const ReservedByte: fn(u8) -> Self = |byte| Self { message: "Reserved byte" };
    const DuplicatedByte: fn(u8) -> Self = |byte| Self { message: "Duplicated byte" };
}

const ALPHABET_SIZE: usize = 64;
const PAD_BYTE: u8 = b'=';

struct Alphabet;

impl Alphabet {
    pub const fn from_str_unchecked(alphabet: &str) -> Self {
        Alphabet
    }

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

        Ok(Self::from_str_unchecked(alphabet))
    }
}

#[test]
fn test_valid_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_length() {
    let alphabet = "short_length";
    let result = Alphabet::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "Invalid length");
}

#[test]
fn test_unprintable_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYYabcdefghijklmnopqrstuvwx0123456789+/"; // 'Y' is duplicated
    let result = Alphabet::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "Duplicated byte");
}

#[test]
fn test_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+="; // '=' is reserved
    let result = Alphabet::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "Reserved byte");
}


