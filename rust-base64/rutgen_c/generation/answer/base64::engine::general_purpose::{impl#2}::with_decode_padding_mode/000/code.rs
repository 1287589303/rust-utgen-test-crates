// Answer 0

#[test]
fn test_with_decode_padding_mode_indifferent() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

#[test]
fn test_with_decode_padding_mode_canonical() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::RequireCanonical);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireCanonical);
}

#[test]
fn test_with_decode_padding_mode_no_padding() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_decode_padding_mode(DecodePaddingMode::RequireNone);
    assert_eq!(updated_config.decode_padding_mode, DecodePaddingMode::RequireNone);
}

#[test]
fn test_with_decode_padding_mode_stays_immutable() {
    let config = GeneralPurposeConfig::new();
    let original_padding_mode = config.decode_padding_mode;
    let _ = config.with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(config.decode_padding_mode, original_padding_mode);
}

