// Answer 0

#[test]
fn test_with_encode_padding_true() {
    struct Config {
        encode_padding: bool,
    }
    
    impl Config {
        const fn new(encode_padding: bool) -> Self {
            Self { encode_padding }
        }

        const fn with_encode_padding(self, padding: bool) -> Self {
            Self {
                encode_padding: padding,
                ..self
            }
        }
    }

    let config = Config::new(false);
    let updated_config = config.with_encode_padding(true);
    assert_eq!(updated_config.encode_padding, true);
}

#[test]
fn test_with_encode_padding_false() {
    struct Config {
        encode_padding: bool,
    }
    
    impl Config {
        const fn new(encode_padding: bool) -> Self {
            Self { encode_padding }
        }

        const fn with_encode_padding(self, padding: bool) -> Self {
            Self {
                encode_padding: padding,
                ..self
            }
        }
    }

    let config = Config::new(true);
    let updated_config = config.with_encode_padding(false);
    assert_eq!(updated_config.encode_padding, false);
}

#[test]
fn test_with_encode_padding_boundary() {
    struct Config {
        encode_padding: bool,
    }
    
    impl Config {
        const fn new(encode_padding: bool) -> Self {
            Self { encode_padding }
        }

        const fn with_encode_padding(self, padding: bool) -> Self {
            Self {
                encode_padding: padding,
                ..self
            }
        }
    }

    let config = Config::new(false);
    let updated_config = config.with_encode_padding(true);
    assert_eq!(updated_config.encode_padding, true);

    let updated_config_again = updated_config.with_encode_padding(false);
    assert_eq!(updated_config_again.encode_padding, false);
}

