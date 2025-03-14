// Answer 0

#[derive(Debug, PartialEq)]
struct ParseAlphabetError {
    message: &'static str,
}

const ALPHABET_SIZE: usize = 64;
const PAD_BYTE: u8 = b'=';

struct Alphabet {
    value: String,
}

impl Alphabet {
    pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
        let bytes = alphabet.as_bytes();
        if bytes.len() != ALPHABET_SIZE {
            return Err(ParseAlphabetError { message: "InvalidLength" });
        }

        {
            let mut index = 0;
            while index < ALPHABET_SIZE {
                let byte = bytes[index];

                if !(byte >= 32_u8 && byte <= 126_u8) {
                    return Err(ParseAlphabetError { message: "UnprintableByte" });
                }
                if byte == PAD_BYTE {
                    return Err(ParseAlphabetError { message: "ReservedByte" });
                }

                let mut probe_index = 0;
                while probe_index < ALPHABET_SIZE {
                    if probe_index == index {
                        probe_index += 1;
                        continue;
                    }

                    let probe_byte = bytes[probe_index];

                    if byte == probe_byte {
                        return Err(ParseAlphabetError { message: "DuplicatedByte" });
                    }

                    probe_index += 1;
                }

                index += 1;
            }
        }

        Ok(Self { value: alphabet.to_string() })
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
    let alphabet = "A";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError { message: "InvalidLength" }));
}

#[test]
fn test_unprintable_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x01";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError { message: "UnprintableByte" }));
}

#[test]
fn test_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError { message: "ReservedByte" }));
}

#[test]
fn test_duplicated_byte() {
    let alphabet = "AABCDEFGHIJKLMNOPQRSTUVWXYYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError { message: "DuplicatedByte" }));
}

