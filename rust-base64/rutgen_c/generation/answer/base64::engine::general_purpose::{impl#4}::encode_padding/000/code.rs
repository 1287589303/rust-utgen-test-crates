// Answer 0

#[test]
fn test_encode_padding_with_pad() {
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireCanonical,
    };
    assert_eq!(config.encode_padding(), true);
}

#[test]
fn test_encode_padding_without_pad() {
    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };
    assert_eq!(config.encode_padding(), false);
}

#[test]
fn test_encode_padding_with_indifferent_padding() {
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };
    assert_eq!(config.encode_padding(), true);
}

