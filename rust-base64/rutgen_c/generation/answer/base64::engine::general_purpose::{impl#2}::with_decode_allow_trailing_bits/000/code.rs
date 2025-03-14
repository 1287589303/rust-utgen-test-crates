// Answer 0

#[test]
fn test_with_decode_allow_trailing_bits_true() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_allow_trailing_bits(true);
    assert_eq!(updated_config.decode_allow_trailing_bits, true);
}

#[test]
fn test_with_decode_allow_trailing_bits_false() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_allow_trailing_bits(false);
    assert_eq!(updated_config.decode_allow_trailing_bits, false);
}

#[test]
fn test_with_decode_allow_trailing_bits_preserves_other_fields() {
    let config = GeneralPurposeConfig::new()
        .with_encode_padding(true)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent);
    
    let updated_config = config.with_decode_allow_trailing_bits(true);
    
    assert_eq!(updated_config.encode_padding, true);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

