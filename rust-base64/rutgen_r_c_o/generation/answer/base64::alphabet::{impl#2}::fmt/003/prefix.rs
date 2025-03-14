// Answer 0

#[test]
fn test_invalid_length_duplicate_byte() {
    let duplicated_byte: u8 = 0x41; // 'A'
    let error = ParseAlphabetError::DuplicatedByte(duplicated_byte);
    let _ = format!("{}", error);
}

#[test]
fn test_control_character_duplicate_byte() {
    let duplicated_byte: u8 = 0x0A; // Control character (Line Feed)
    let error = ParseAlphabetError::DuplicatedByte(duplicated_byte);
    let _ = format!("{}", error);
}

#[test]
fn test_printable_character_duplicate_byte() {
    let duplicated_byte: u8 = 0x3F; // '?'
    let error = ParseAlphabetError::DuplicatedByte(duplicated_byte);
    let _ = format!("{}", error);
}

#[test]
fn test_delete_character_duplicate_byte() {
    let duplicated_byte: u8 = 0x7F; // Delete
    let error = ParseAlphabetError::DuplicatedByte(duplicated_byte);
    let _ = format!("{}", error);
}

#[test]
fn test_out_of_bounds_duplicate_byte() {
    let duplicated_byte: u8 = 0x80; // First byte above valid ASCII range
    let error = ParseAlphabetError::DuplicatedByte(duplicated_byte);
    let _ = format!("{}", error);
}

