// Answer 0

#[test]
fn test_encode_padding_true() {
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireCanonical,
    };
    assert!(config.encode_padding());
}

#[test]
fn test_encode_padding_false() {
    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };
    assert!(!config.encode_padding());
}

#[test]
fn test_encode_padding_indifferent() {
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };
    assert!(config.encode_padding());
} 

#[test]
fn test_encode_padding_no_padding() {
    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };
    assert!(!config.encode_padding());
}

