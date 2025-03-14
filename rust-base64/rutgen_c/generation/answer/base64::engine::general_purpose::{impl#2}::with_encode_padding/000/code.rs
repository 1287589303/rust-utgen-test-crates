// Answer 0

#[test]
fn test_with_encode_padding_true() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(true);
    assert!(updated_config.encode_padding);
}

#[test]
fn test_with_encode_padding_false() {
    let config = GeneralPurposeConfig::new();
    let updated_config = config.with_encode_padding(false);
    assert!(!updated_config.encode_padding);
}

#[test]
fn test_with_encode_padding_consistent() {
    let config = GeneralPurposeConfig::new().with_encode_padding(false);
    let updated_config = config.with_encode_padding(false);
    assert_eq!(updated_config.encode_padding, config.encode_padding);
}

#[test]
fn test_with_encode_padding_chaining() {
    let config = GeneralPurposeConfig::new()
        .with_encode_padding(true)
        .with_decode_allow_trailing_bits(true);
    let updated_config = config.with_encode_padding(false);
    assert!(config.encode_padding);
    assert!(!updated_config.encode_padding);
}

