// Answer 0

#[derive(Debug, PartialEq)]
struct GeneralPurposeConfig;

impl GeneralPurposeConfig {
    fn new() -> Self {
        GeneralPurposeConfig
    }

    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_general_purpose_config_default() {
    let config = GeneralPurposeConfig::default();
    assert_eq!(config, GeneralPurposeConfig::new());
}

#[test]
fn test_general_purpose_config_new() {
    let config_new = GeneralPurposeConfig::new();
    assert_eq!(config_new, GeneralPurposeConfig);
}

