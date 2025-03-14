// Answer 0

#[derive(Debug)]
struct Config {
    value: String,
}

struct SampleStruct {
    config: Config,
}

impl SampleStruct {
    fn config(&self) -> &Config {
        &self.config
    }
}

#[test]
fn test_config_returns_reference() {
    let sample = SampleStruct {
        config: Config {
            value: String::from("test_value"),
        },
    };
    let config_reference = sample.config();
    assert_eq!(config_reference.value, "test_value");
}

#[test]
fn test_config_empty_string() {
    let sample = SampleStruct {
        config: Config {
            value: String::from(""),
        },
    };
    let config_reference = sample.config();
    assert_eq!(config_reference.value, "");
}

#[test]
fn test_config_special_characters() {
    let sample = SampleStruct {
        config: Config {
            value: String::from("!@#$%^&*()"),
        },
    };
    let config_reference = sample.config();
    assert_eq!(config_reference.value, "!@#$%^&*()");
}

