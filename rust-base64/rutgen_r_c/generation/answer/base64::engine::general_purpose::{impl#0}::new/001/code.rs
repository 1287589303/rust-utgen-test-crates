// Answer 0

#[test]
fn test_general_purpose_new_standard() {
    let alphabet = Alphabet { symbols: [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', 
        b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', 
        b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', 
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', 
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', 
        b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', 
        b'8', b'9', b'+', b'/', 
    ]};
    let config = PAD;
    let engine = GeneralPurpose::new(&alphabet, config);
    
    assert_eq!(engine.config.encode_padding, config.encode_padding);
    assert_eq!(engine.config.decode_padding_mode, config.decode_padding_mode);
    assert_eq!(engine.encode_table, encode_table(&alphabet));
    assert_eq!(engine.decode_table, decode_table(&alphabet));
}

#[test]
fn test_general_purpose_new_url_safe() {
    let alphabet = Alphabet { symbols: [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', 
        b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', 
        b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', 
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', 
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', 
        b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', 
        b'8', b'9', b'-', b'_', 
    ]};
    let config = PAD;
    let engine = GeneralPurpose::new(&alphabet, config);
    
    assert_eq!(engine.config.encode_padding, config.encode_padding);
    assert_eq!(engine.config.decode_padding_mode, config.decode_padding_mode);
    assert_eq!(engine.encode_table, encode_table(&alphabet));
    assert_eq!(engine.decode_table, decode_table(&alphabet));
}

