// Answer 0

#[test]
fn test_with_encode_padding_true() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(true);
    assert_eq!(updated_config.encode_padding, true);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_with_encode_padding_false() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(false);
    assert_eq!(updated_config.encode_padding, false);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_with_encode_padding_changes_only_padding() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(false);
    assert_ne!(updated_config.encode_padding, config.encode_padding);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

