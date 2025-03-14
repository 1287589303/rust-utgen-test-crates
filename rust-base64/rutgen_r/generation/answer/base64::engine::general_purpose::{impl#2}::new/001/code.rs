// Answer 0

#[test]
fn test_new_config_with_default_values() {
    struct Config {
        encode_padding: bool,
        decode_allow_trailing_bits: bool,
        decode_padding_mode: DecodePaddingMode,
    }

    enum DecodePaddingMode {
        RequireCanonical,
    }

    impl Config {
        pub const fn new() -> Self {
            Self {
                encode_padding: true,
                decode_allow_trailing_bits: false,
                decode_padding_mode: DecodePaddingMode::RequireCanonical,
            }
        }
    }

    let config = Config::new();
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    match config.decode_padding_mode {
        DecodePaddingMode::RequireCanonical => {}
    }
}

