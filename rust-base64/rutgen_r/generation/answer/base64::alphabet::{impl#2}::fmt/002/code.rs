// Answer 0

#[derive(Debug)]
enum AlphabetError {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}

impl std::fmt::Display for AlphabetError {
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
fn test_unprintable_byte() {
    let byte: u8 = 0x01; // Example unprintable byte
    let error = AlphabetError::UnprintableByte(byte);
    assert_eq!(format!("{}", error), "Unprintable byte: 0x01");
}

#[test]
fn test_another_unprintable_byte() {
    let byte: u8 = 0x7F; // Another example unprintable byte
    let error = AlphabetError::UnprintableByte(byte);
    assert_eq!(format!("{}", error), "Unprintable byte: 0x7f");
}

