// Answer 0

#[derive(Debug)]
enum DecodeError {
    InvalidByte(usize, u8),
    InvalidLength(usize),
    InvalidLastSymbol(usize, u8),
    InvalidPadding,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::InvalidByte(index, byte) => {
                write!(f, "Invalid symbol {}, offset {}.", byte, index)
            }
            Self::InvalidLength(len) => write!(f, "Invalid input length: {}", len),
            Self::InvalidLastSymbol(index, byte) => {
                write!(f, "Invalid last symbol {}, offset {}.", byte, index)
            }
            Self::InvalidPadding => write!(f, "Invalid padding"),
        }
    }
}

#[test]
fn test_invalid_byte_fmt() {
    let error = DecodeError::InvalidByte(5, 255);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid symbol 255, offset 5.");
}

#[test]
fn test_invalid_length_fmt() {
    let error = DecodeError::InvalidLength(10);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid input length: 10");
}

#[test]
fn test_invalid_last_symbol_fmt() {
    let error = DecodeError::InvalidLastSymbol(3, 128);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid last symbol 128, offset 3.");
}

#[test]
fn test_invalid_padding_fmt() {
    let error = DecodeError::InvalidPadding;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid padding");
}

