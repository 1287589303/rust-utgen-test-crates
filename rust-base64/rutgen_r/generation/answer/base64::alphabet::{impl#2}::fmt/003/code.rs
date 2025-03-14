// Answer 0

#[derive(Debug)]
enum Alphabet {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}

impl std::fmt::Display for Alphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid length - must be 64 bytes"),
            Self::DuplicatedByte(b) => write!(f, "Duplicated byte: {:#04x}", b),
            Self::UnprintableByte(b) => write!(f, "Unprintable byte: {:#04x}", b),
            Self::ReservedByte(b) => write!(f, "Reserved byte: {:#04x}", b),
        }
    }
}

#[test]
fn test_display_duplicated_byte() {
    let byte_value: u8 = 0xFF;
    let alphabet_instance = Alphabet::DuplicatedByte(byte_value);
    let output = format!("{}", alphabet_instance);
    assert_eq!(output, "Duplicated byte: 0xff");
}

#[test]
fn test_display_another_duplicated_byte() {
    let byte_value: u8 = 0x01;
    let alphabet_instance = Alphabet::DuplicatedByte(byte_value);
    let output = format!("{}", alphabet_instance);
    assert_eq!(output, "Duplicated byte: 0x01");
}

