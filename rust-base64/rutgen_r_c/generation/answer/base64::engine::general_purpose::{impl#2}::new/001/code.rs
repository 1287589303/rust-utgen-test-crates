// Answer 0

#[test]
fn test_new_general_purpose_config() {
    let config = GeneralPurposeConfig::new();
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

