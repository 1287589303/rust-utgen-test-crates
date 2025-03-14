// Answer 0

#[test]
fn test_general_purpose_config_default() {
    let config = GeneralPurposeConfig::default();
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_general_purpose_config_new() {
    let config = GeneralPurposeConfig::new();
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

