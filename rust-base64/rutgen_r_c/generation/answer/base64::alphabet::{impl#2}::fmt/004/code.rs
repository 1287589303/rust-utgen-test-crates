// Answer 0

#[test]
fn test_invalid_length_display() {
    let error = ParseAlphabetError::InvalidLength;
    let mut output = String::new();
    
    write!(&mut output, "{}", error).unwrap();
    
    assert_eq!(output, "Invalid length - must be 64 bytes");
}

#[test]
fn test_duplicated_byte_display() {
    let error = ParseAlphabetError::DuplicatedByte(0x42);
    let mut output = String::new();
    
    write!(&mut output, "{}", error).unwrap();
    
    assert_eq!(output, "Duplicated byte: 0x42");
}

#[test]
fn test_unprintable_byte_display() {
    let error = ParseAlphabetError::UnprintableByte(0x1F);
    let mut output = String::new();
    
    write!(&mut output, "{}", error).unwrap();
    
    assert_eq!(output, "Unprintable byte: 0x1f");
}

#[test]
fn test_reserved_byte_display() {
    let error = ParseAlphabetError::ReservedByte(0x3D);
    let mut output = String::new();
    
    write!(&mut output, "{}", error).unwrap();
    
    assert_eq!(output, "Reserved byte: 0x3d");
}

