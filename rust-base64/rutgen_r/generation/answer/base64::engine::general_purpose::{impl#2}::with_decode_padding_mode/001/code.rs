// Answer 0

#[derive(Debug, Clone)]
struct Config {
    decode_padding_mode: DecodePaddingMode,
}

#[derive(Debug, Clone)]
enum DecodePaddingMode {
    RequireCanonicalPadding,
    RequireNoPadding,
    Indifferent,
}

impl Config {
    pub const fn with_decode_padding_mode(self, mode: DecodePaddingMode) -> Self {
        Self {
            decode_padding_mode: mode,
            ..self
        }
    }
}

#[test]
fn test_with_decode_padding_mode_canonical() {
    let config = Config {
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };
    let new_config = config.clone().with_decode_padding_mode(DecodePaddingMode::RequireCanonicalPadding);
    assert_eq!(new_config.decode_padding_mode, DecodePaddingMode::RequireCanonicalPadding);
}

#[test]
fn test_with_decode_padding_mode_no_padding() {
    let config = Config {
        decode_padding_mode: DecodePaddingMode::RequireCanonicalPadding,
    };
    let new_config = config.clone().with_decode_padding_mode(DecodePaddingMode::RequireNoPadding);
    assert_eq!(new_config.decode_padding_mode, DecodePaddingMode::RequireNoPadding);
}

#[test]
fn test_with_decode_padding_mode_indifferent() {
    let config = Config {
        decode_padding_mode: DecodePaddingMode::RequireNoPadding,
    };
    let new_config = config.clone().with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(new_config.decode_padding_mode, DecodePaddingMode::Indifferent);
}

