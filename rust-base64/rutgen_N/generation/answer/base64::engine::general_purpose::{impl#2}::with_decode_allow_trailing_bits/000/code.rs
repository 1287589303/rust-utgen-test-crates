// Answer 0

#[test]
fn test_with_decode_allow_trailing_bits() {
    struct Config {
        decode_allow_trailing_bits: bool,
    }

    impl Config {
        pub const fn new() -> Self {
            Self {
                decode_allow_trailing_bits: false,
            }
        }

        pub const fn with_decode_allow_trailing_bits(self, allow: bool) -> Self {
            Self {
                decode_allow_trailing_bits: allow,
                ..self
            }
        }
    }

    let default_config = Config::new();
    let updated_config = default_config.with_decode_allow_trailing_bits(true);
    assert!(updated_config.decode_allow_trailing_bits);

    let another_updated_config = default_config.with_decode_allow_trailing_bits(false);
    assert!(!another_updated_config.decode_allow_trailing_bits);
}

