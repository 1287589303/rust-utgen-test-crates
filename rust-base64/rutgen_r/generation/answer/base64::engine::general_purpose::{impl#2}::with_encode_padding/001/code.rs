// Answer 0

#[derive(Clone)]
struct Config {
    encode_padding: bool,
}

impl Config {
    pub const fn with_encode_padding(self, padding: bool) -> Self {
        Self {
            encode_padding: padding,
            ..self
        }
    }
}

#[test]
fn test_with_encode_padding_true() {
    let config = Config { encode_padding: false };
    let new_config = config.clone().with_encode_padding(true);
    assert!(new_config.encode_padding);
}

#[test]
fn test_with_encode_padding_false() {
    let config = Config { encode_padding: true };
    let new_config = config.clone().with_encode_padding(false);
    assert!(!new_config.encode_padding);
}

#[test]
fn test_with_encode_padding_no_change() {
    let config = Config { encode_padding: false };
    let new_config = config.with_encode_padding(false);
    assert!(!new_config.encode_padding);
}

