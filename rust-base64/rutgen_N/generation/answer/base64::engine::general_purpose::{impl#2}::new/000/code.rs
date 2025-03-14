// Answer 0

#[derive(Debug)]
struct DecodePaddingMode {
    mode: String,
}

impl DecodePaddingMode {
    const RequireCanonical: Self = DecodePaddingMode {
        mode: String::from("RequireCanonical"),
    };
}

struct Config {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
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

#[test]
fn test_config_new() {
    let config = Config::new();
    assert_eq!(config.encode_padding, true);
    assert_eq!(config.decode_allow_trailing_bits, false);
    assert_eq!(config.decode_padding_mode.mode, "RequireCanonical");
}

