// Answer 0

#[test]
fn test_default() {
    struct GeneralPurposeConfig;

    impl GeneralPurposeConfig {
        fn new() -> Self {
            GeneralPurposeConfig
        }

        fn default() -> Self {
            Self::new()
        }
    }

    let config = GeneralPurposeConfig::default();
    assert!(std::mem::size_of_val(&config) > 0);
}

