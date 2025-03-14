// Answer 0

#[derive(Clone)]
struct Config {
    decode_padding_mode: DecodePaddingMode,
}

#[derive(Clone)]
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
    assert_eq!(matches!(new_config.decode_padding_mode, DecodePaddingMode::RequireCanonicalPadding), true);
}

#[test]
fn test_with_decode_padding_mode_no_padding() {
    let config = Config {
        decode_padding_mode: DecodePaddingMode::RequireCanonicalPadding,
    };
    let new_config = config.clone().with_decode_padding_mode(DecodePaddingMode::RequireNoPadding);
    assert_eq!(matches!(new_config.decode_padding_mode, DecodePaddingMode::RequireNoPadding), true);
}

#[test]
fn test_with_decode_padding_mode_indifferent() {
    let config = Config {
        decode_padding_mode: DecodePaddingMode::RequireNoPadding,
    };
    let new_config = config.clone().with_decode_padding_mode(DecodePaddingMode::Indifferent);
    assert_eq!(matches!(new_config.decode_padding_mode, DecodePaddingMode::Indifferent), true);
}

